## Getting Started

Clone this repository and install its dependencies:


## Building the project

After any changes to the contract, run:

```bash
npm run build
# or
yarn build
```

## Testing the project

To run the tests in solidity, You'll need to comment out this function in the test contract `testDropWithCustomTokens` before running tests:
	```bash
	
	forge test
	```

To run the tests in rust, uncomment `testDropWithCustomTokens`, `cd` into `forge_tests` directory and run:
	```bash
	cargo test
	```
To run rust tests with the println statements, run:
	```bash
	cargo test -- --nocapture
	```

