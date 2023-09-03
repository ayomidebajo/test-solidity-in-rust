use ethers::{
    abi::{Detokenize, Tokenize},
    solc::{Project, ProjectCompileOutput, ProjectPathsConfig},
    types::U256,
};
use forge::{
    executor::{
        inspector::CheatsConfig,
        opts::{Env, EvmOpts},
    },
    result::TestSetup,
    ContractRunner, MultiContractRunner, MultiContractRunnerBuilder,
};
use foundry_config::{fs_permissions::PathPermission, Config, FsPermissions};
use foundry_evm::executor::{Backend, EvmError, ExecutorBuilder, SpecId};
use once_cell::sync::Lazy;

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

static PROJECT: Lazy<Project> = Lazy::new(|| {
    //  detect Cargo.toml
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root = PathBuf::from(root.parent().unwrap());
    // detect the path where our smart contract live
    let paths = ProjectPathsConfig::builder()
        .root(root.clone())
        .build()
        .unwrap();
    // build the path
    Project::builder()
        .paths(paths)
        .ephemeral()
        .no_artifacts()
        .build()
        .unwrap()
});

// These are the env configurations needed by forge
static EVM_OPTS: Lazy<EvmOpts> = Lazy::new(|| EvmOpts {
    env: Env {
        gas_limit: 18446744073709551615,
        chain_id: Some(foundry_common::DEV_CHAIN_ID),
        tx_origin: Config::DEFAULT_SENDER,
        block_number: 1,
        block_timestamp: 1,
        ..Default::default()
    },
    sender: Config::DEFAULT_SENDER,
    initial_balance: U256::MAX,
    ffi: true,
    memory_limit: 2u64.pow(24),
    ..Default::default()
});

static COMPILED: Lazy<ProjectCompileOutput> = Lazy::new(|| {
    let out = (*PROJECT).compile().unwrap();
    if out.has_compiler_errors() {
        eprintln!("{out}");
        panic!("Compiled with errors");
    }
    out
});

/// Builds a base runner
fn base_runner() -> MultiContractRunnerBuilder {
    // Builds the base runner with a configured address
    MultiContractRunnerBuilder::default().sender(EVM_OPTS.sender)
}

fn manifest_root() -> PathBuf {
    let mut root = Path::new(env!("CARGO_MANIFEST_DIR"));
    // need to check here where we're executing the test from, if in `forge` we need to also allow
    // `testdata`
    if root.ends_with("forge") {
        root = root.parent().unwrap();
    }
    root.to_path_buf()
}

/// Builds a non-tracing runner
fn runner_with_config(mut config: Config) -> MultiContractRunner {
    // add cargo 
    config.allow_paths.push(manifest_root());

    base_runner()
        .with_cheats_config(CheatsConfig::new(&config, &EVM_OPTS))
        .sender(config.sender)
        .evm_spec(SpecId::SHANGHAI)
        .build(
            &PROJECT.paths.root,
            (*COMPILED).clone(),
            EVM_OPTS.local_evm_env(),
            EVM_OPTS.clone(),
        )
        .unwrap()
}

/// Builds a non-tracing runner
pub fn runner() -> MultiContractRunner {
    // more project configurations
    let mut config = Config::with_root(PROJECT.root());
    // permissions needed when accessing the path. we detect all smart contracts in the workspace here
    config.fs_permissions = FsPermissions::new(vec![PathPermission::read_write(manifest_root())]);
    runner_with_config(config)
}

// executes the call to a smart contract with the arguments specifed to the smart contract
pub async fn execute<T, R>(
    runner: &mut MultiContractRunner,
    contract_name: &'static str,
    fn_name: &'static str,
    args: T,
) -> Result<R, EvmError>
where
    T: Tokenize,
    R: Detokenize + Debug,
{
    // This is where our local EVM lives (i.e our local EVM implementation)
    let db = Backend::spawn(runner.fork.take()).await;

    // this is where we extract, the artifacts, abis and byte code from the compiled smart contract
    let (id, (abi, deploy_code, libs)) = runner
        .contracts
        .iter()
        .find(|(id, (abi, _, _))| id.name == contract_name && abi.functions.contains_key(fn_name))
        .unwrap();

    // returns an identified artifact
    let identifier = id.identifier();

    // gets a smart contract function
    let function = abi.functions.get(fn_name).unwrap().first().unwrap().clone();

    // the executor executes the smart contract calls
    let executor = ExecutorBuilder::default()
        .with_cheatcodes(runner.cheats_config.clone())
        .with_config(runner.env.clone())
        .with_spec(runner.evm_spec)
        .with_gas_limit(runner.evm_opts.gas_limit())
        .set_tracing(runner.evm_opts.verbosity >= 3)
        .set_coverage(runner.coverage)
        .build(db.clone());

    // detects a single contract on the EVM and gives you powers(methods) to call on the contract inside.
    let mut single_runner = ContractRunner::new(
        &identifier,
        executor,
        abi,
        deploy_code.clone(),
        runner.evm_opts.initial_balance,
        runner.sender,
        runner.errors.as_ref(),
        libs,
    );

    // deploys the single contract inside the runner from a sending account
    let setup = single_runner.setup(true);
    // extracts the smart contract address
    let TestSetup { address, .. } = setup;

    // executes the contract call
    let result = single_runner.executor.execute_test::<R, _, _>(
        single_runner.sender,
        address,
        function,
        args,
        0.into(),
        single_runner.errors,
    )?;

    // we should see this when we run tests individually
    println!("Smart contract function name called {fn_name}: Gas used {:#?}", result.gas_used);

    Ok(result.result)
}
