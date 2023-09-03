// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

// imported it this way, because it's being detected for some reason, you can use the other way if you want
import "node_modules/@thirdweb-dev/contracts/base/ERC721Drop.sol";

contract MyContract is ERC721Drop {
    constructor(
        address _defaultAdmin,
        string memory _name,
        string memory _symbol,
        address _royaltyRecipient,
        uint128 _royaltyBps,
        address _primarySaleRecipient
    )
        ERC721Drop(
            _defaultAdmin,
            _name,
            _symbol,
            _royaltyRecipient,
            _royaltyBps,
            _primarySaleRecipient
        )
    {}

    function mint(address _to, uint256 _amount) external {
    require(_amount > 0, 'You must mint at least one token!');
    _safeMint(_to, _amount);
}
}