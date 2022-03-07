pub use nolosscollateralauction_mod::*;
#[allow(clippy::too_many_arguments)]
mod nolosscollateralauction_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "NoLossCollateralAuction was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static NOLOSSCOLLATERALAUCTION_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"codex_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"limes_\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"Guarded__notGranted\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"Guarded__notRoot\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"x\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"y\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Math__add_overflow\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"x\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"y\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Math__mul_overflow\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"x\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"y\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Math__sub_overflow\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__cancelAuction_notRunningAction\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__checkReentrancy_reentered\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__init_vaultAlreadyInit\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__isStopped_stoppedIncorrect\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__redoAuction_cannotReset\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__redoAuction_notRunningAuction\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__redoAuction_zeroStartPrice\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__setParam_unrecognizedParam\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__startAuction_overflow\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__startAuction_zeroCollateralToSell\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__startAuction_zeroDebt\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__startAuction_zeroStartPrice\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__startAuction_zeroUser\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__takeCollateral_needsReset\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__takeCollateral_noPartialPurchase\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__takeCollateral_notRunningAuction\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoLossCollateralAuction__takeCollateral_tooExpensive\",\n    \"type\": \"error\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"sig\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"who\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"AllowCaller\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"sig\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"who\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"BlockCaller\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"Init\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"auctionId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"startPrice\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"debt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralToSell\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"keeper\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"tip\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"RedoAuction\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"auctionId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"startPrice\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"debt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralToSell\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"keeper\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"tip\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"StartAuction\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"auctionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"StopAuction\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"auctionId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"maxPrice\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"price\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"owe\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"debt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralToSell\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"TakeCollateral\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"auctionDebtFloor\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"UpdateAuctionDebtFloor\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ANY_CALLER\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ANY_SIG\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"activeAuctions\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"aer\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IAer\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"sig\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"who\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowCaller\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"auctionCounter\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"auctions\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"debt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralToSell\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint96\",\n        \"name\": \"startsAt\",\n        \"type\": \"uint96\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startPrice\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"sig\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"who\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"blockCaller\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"sig\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"who\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"canCall\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"auctionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"cancelAuction\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"codex\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ICodex\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"count\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeTip\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"\",\n        \"type\": \"uint64\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"flatTip\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint192\",\n        \"name\": \"\",\n        \"type\": \"uint192\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"auctionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getStatus\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"needsRedo\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"price\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralToSell\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"debt\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"collybus\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"init\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"limes\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ILimes\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"list\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"auctionId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"keeper\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"redoAuction\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"param\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"data\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setParam\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"param\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"data\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setParam\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"param\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"data\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setParam\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"param\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"data\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setParam\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"debt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralToSell\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"keeper\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"startAuction\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"auctionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"stopped\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"auctionId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maxPrice\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"takeCollateral\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"updateAuctionDebtFloor\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"vaults\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"multiplier\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maxAuctionDuration\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"auctionDebtFloor\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"contract ICollybus\",\n        \"name\": \"collybus\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IPriceCalculator\",\n        \"name\": \"calculator\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct NoLossCollateralAuction<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for NoLossCollateralAuction<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for NoLossCollateralAuction<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(NoLossCollateralAuction))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> NoLossCollateralAuction<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                NOLOSSCOLLATERALAUCTION_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `ANY_CALLER` (0xbbd91c46) function"]
        pub fn any_caller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([187, 217, 28, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ANY_SIG` (0x2936ff2b) function"]
        pub fn any_sig(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([41, 54, 255, 43], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `activeAuctions` (0x790d8596) function"]
        pub fn active_auctions(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([121, 13, 133, 150], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `aer` (0xb748b9fb) function"]
        pub fn aer(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([183, 72, 185, 251], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowCaller` (0xa746d489) function"]
        pub fn allow_caller(
            &self,
            sig: [u8; 32],
            who: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 70, 212, 137], (sig, who))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `auctionCounter` (0xa7e76644) function"]
        pub fn auction_counter(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([167, 231, 102, 68], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `auctions` (0x571a26a0) function"]
        pub fn auctions(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::Address,
                u128,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([87, 26, 38, 160], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `blockCaller` (0x012abbe9) function"]
        pub fn block_caller(
            &self,
            sig: [u8; 32],
            who: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 42, 187, 233], (sig, who))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `canCall` (0x52b43adf) function"]
        pub fn can_call(
            &self,
            sig: [u8; 32],
            who: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([82, 180, 58, 223], (sig, who))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelAuction` (0x96b5a755) function"]
        pub fn cancel_auction(
            &self,
            auction_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 181, 167, 85], auction_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `codex` (0x41779f86) function"]
        pub fn codex(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([65, 119, 159, 134], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `count` (0x06661abd) function"]
        pub fn count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([6, 102, 26, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeTip` (0x944830d3) function"]
        pub fn fee_tip(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([148, 72, 48, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `flatTip` (0x970b48f5) function"]
        pub fn flat_tip(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([151, 11, 72, 245], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getStatus` (0x5c622a0e) function"]
        pub fn get_status(
            &self,
            auction_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([92, 98, 42, 14], auction_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `init` (0xf09a4016) function"]
        pub fn init(
            &self,
            vault: ethers::core::types::Address,
            collybus: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 154, 64, 22], (vault, collybus))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `limes` (0x9f92b96e) function"]
        pub fn limes(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([159, 146, 185, 110], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `list` (0x0f560cd7) function"]
        pub fn list(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([15, 86, 12, 215], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redoAuction` (0x2a2e0006) function"]
        pub fn redo_auction(
            &self,
            auction_id: ethers::core::types::U256,
            keeper: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([42, 46, 0, 6], (auction_id, keeper))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setParam` (0x2901a27e) function"]
        pub fn set_param_1(
            &self,
            vault: ethers::core::types::Address,
            param: [u8; 32],
            data: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 1, 162, 126], (vault, param, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setParam` (0x9f30490a) function"]
        pub fn set_param_0(
            &self,
            param: [u8; 32],
            data: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 48, 73, 10], (param, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setParam` (0xedd1b0be) function"]
        pub fn set_param_3(
            &self,
            param: [u8; 32],
            data: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 209, 176, 190], (param, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setParam` (0xf63b0f3d) function"]
        pub fn set_param_4(
            &self,
            vault: ethers::core::types::Address,
            param: [u8; 32],
            data: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 59, 15, 61], (vault, param, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startAuction` (0x15c48687) function"]
        pub fn start_auction(
            &self,
            debt: ethers::core::types::U256,
            collateral_to_sell: ethers::core::types::U256,
            vault: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
            user: ethers::core::types::Address,
            keeper: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [21, 196, 134, 135],
                    (debt, collateral_to_sell, vault, token_id, user, keeper),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stopped` (0x75f12b21) function"]
        pub fn stopped(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([117, 241, 43, 33], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `takeCollateral` (0x1ad04b75) function"]
        pub fn take_collateral(
            &self,
            auction_id: ethers::core::types::U256,
            collateral_amount: ethers::core::types::U256,
            max_price: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [26, 208, 75, 117],
                    (auction_id, collateral_amount, max_price, recipient, data),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateAuctionDebtFloor` (0x0f467d82) function"]
        pub fn update_auction_debt_floor(
            &self,
            vault: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 70, 125, 130], vault)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vaults` (0xa622ee7c) function"]
        pub fn vaults(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::Address,
                ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([166, 34, 238, 124], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AllowCaller` event"]
        pub fn allow_caller_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AllowCallerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BlockCaller` event"]
        pub fn block_caller_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BlockCallerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Init` event"]
        pub fn init_filter(&self) -> ethers::contract::builders::Event<M, InitFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RedoAuction` event"]
        pub fn redo_auction_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RedoAuctionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `StartAuction` event"]
        pub fn start_auction_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StartAuctionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `StopAuction` event"]
        pub fn stop_auction_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StopAuctionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TakeCollateral` event"]
        pub fn take_collateral_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TakeCollateralFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdateAuctionDebtFloor` event"]
        pub fn update_auction_debt_floor_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdateAuctionDebtFloorFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, NoLossCollateralAuctionEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "AllowCaller", abi = "AllowCaller(bytes32,address)")]
    pub struct AllowCallerFilter {
        pub sig: [u8; 32],
        pub who: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "BlockCaller", abi = "BlockCaller(bytes32,address)")]
    pub struct BlockCallerFilter {
        pub sig: [u8; 32],
        pub who: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Init", abi = "Init(address)")]
    pub struct InitFilter {
        pub vault: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "RedoAuction",
        abi = "RedoAuction(uint256,uint256,uint256,uint256,address,uint256,address,address,uint256)"
    )]
    pub struct RedoAuctionFilter {
        #[ethevent(indexed)]
        pub auction_id: ethers::core::types::U256,
        pub start_price: ethers::core::types::U256,
        pub debt: ethers::core::types::U256,
        pub collateral_to_sell: ethers::core::types::U256,
        pub vault: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub keeper: ethers::core::types::Address,
        pub tip: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "StartAuction",
        abi = "StartAuction(uint256,uint256,uint256,uint256,address,uint256,address,address,uint256)"
    )]
    pub struct StartAuctionFilter {
        #[ethevent(indexed)]
        pub auction_id: ethers::core::types::U256,
        pub start_price: ethers::core::types::U256,
        pub debt: ethers::core::types::U256,
        pub collateral_to_sell: ethers::core::types::U256,
        pub vault: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub keeper: ethers::core::types::Address,
        pub tip: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "StopAuction", abi = "StopAuction(uint256)")]
    pub struct StopAuctionFilter {
        pub auction_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "TakeCollateral",
        abi = "TakeCollateral(uint256,uint256,uint256,uint256,uint256,uint256,address,uint256,address)"
    )]
    pub struct TakeCollateralFilter {
        #[ethevent(indexed)]
        pub auction_id: ethers::core::types::U256,
        pub max_price: ethers::core::types::U256,
        pub price: ethers::core::types::U256,
        pub owe: ethers::core::types::U256,
        pub debt: ethers::core::types::U256,
        pub collateral_to_sell: ethers::core::types::U256,
        pub vault: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "UpdateAuctionDebtFloor",
        abi = "UpdateAuctionDebtFloor(address,uint256)"
    )]
    pub struct UpdateAuctionDebtFloorFilter {
        #[ethevent(indexed)]
        pub vault: ethers::core::types::Address,
        pub auction_debt_floor: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum NoLossCollateralAuctionEvents {
        AllowCallerFilter(AllowCallerFilter),
        BlockCallerFilter(BlockCallerFilter),
        InitFilter(InitFilter),
        RedoAuctionFilter(RedoAuctionFilter),
        StartAuctionFilter(StartAuctionFilter),
        StopAuctionFilter(StopAuctionFilter),
        TakeCollateralFilter(TakeCollateralFilter),
        UpdateAuctionDebtFloorFilter(UpdateAuctionDebtFloorFilter),
    }
    impl ethers::contract::EthLogDecode for NoLossCollateralAuctionEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AllowCallerFilter::decode_log(log) {
                return Ok(NoLossCollateralAuctionEvents::AllowCallerFilter(decoded));
            }
            if let Ok(decoded) = BlockCallerFilter::decode_log(log) {
                return Ok(NoLossCollateralAuctionEvents::BlockCallerFilter(decoded));
            }
            if let Ok(decoded) = InitFilter::decode_log(log) {
                return Ok(NoLossCollateralAuctionEvents::InitFilter(decoded));
            }
            if let Ok(decoded) = RedoAuctionFilter::decode_log(log) {
                return Ok(NoLossCollateralAuctionEvents::RedoAuctionFilter(decoded));
            }
            if let Ok(decoded) = StartAuctionFilter::decode_log(log) {
                return Ok(NoLossCollateralAuctionEvents::StartAuctionFilter(decoded));
            }
            if let Ok(decoded) = StopAuctionFilter::decode_log(log) {
                return Ok(NoLossCollateralAuctionEvents::StopAuctionFilter(decoded));
            }
            if let Ok(decoded) = TakeCollateralFilter::decode_log(log) {
                return Ok(NoLossCollateralAuctionEvents::TakeCollateralFilter(decoded));
            }
            if let Ok(decoded) = UpdateAuctionDebtFloorFilter::decode_log(log) {
                return Ok(NoLossCollateralAuctionEvents::UpdateAuctionDebtFloorFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for NoLossCollateralAuctionEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                NoLossCollateralAuctionEvents::AllowCallerFilter(element) => element.fmt(f),
                NoLossCollateralAuctionEvents::BlockCallerFilter(element) => element.fmt(f),
                NoLossCollateralAuctionEvents::InitFilter(element) => element.fmt(f),
                NoLossCollateralAuctionEvents::RedoAuctionFilter(element) => element.fmt(f),
                NoLossCollateralAuctionEvents::StartAuctionFilter(element) => element.fmt(f),
                NoLossCollateralAuctionEvents::StopAuctionFilter(element) => element.fmt(f),
                NoLossCollateralAuctionEvents::TakeCollateralFilter(element) => element.fmt(f),
                NoLossCollateralAuctionEvents::UpdateAuctionDebtFloorFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `ANY_CALLER`function with signature `ANY_CALLER()` and selector `[187, 217, 28, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ANY_CALLER", abi = "ANY_CALLER()")]
    pub struct AnyCallerCall;
    #[doc = "Container type for all input parameters for the `ANY_SIG`function with signature `ANY_SIG()` and selector `[41, 54, 255, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ANY_SIG", abi = "ANY_SIG()")]
    pub struct AnySigCall;
    #[doc = "Container type for all input parameters for the `activeAuctions`function with signature `activeAuctions(uint256)` and selector `[121, 13, 133, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "activeAuctions", abi = "activeAuctions(uint256)")]
    pub struct ActiveAuctionsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `aer`function with signature `aer()` and selector `[183, 72, 185, 251]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "aer", abi = "aer()")]
    pub struct AerCall;
    #[doc = "Container type for all input parameters for the `allowCaller`function with signature `allowCaller(bytes32,address)` and selector `[167, 70, 212, 137]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowCaller", abi = "allowCaller(bytes32,address)")]
    pub struct AllowCallerCall {
        pub sig: [u8; 32],
        pub who: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `auctionCounter`function with signature `auctionCounter()` and selector `[167, 231, 102, 68]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "auctionCounter", abi = "auctionCounter()")]
    pub struct AuctionCounterCall;
    #[doc = "Container type for all input parameters for the `auctions`function with signature `auctions(uint256)` and selector `[87, 26, 38, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "auctions", abi = "auctions(uint256)")]
    pub struct AuctionsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `blockCaller`function with signature `blockCaller(bytes32,address)` and selector `[1, 42, 187, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "blockCaller", abi = "blockCaller(bytes32,address)")]
    pub struct BlockCallerCall {
        pub sig: [u8; 32],
        pub who: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `canCall`function with signature `canCall(bytes32,address)` and selector `[82, 180, 58, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "canCall", abi = "canCall(bytes32,address)")]
    pub struct CanCallCall {
        pub sig: [u8; 32],
        pub who: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `cancelAuction`function with signature `cancelAuction(uint256)` and selector `[150, 181, 167, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "cancelAuction", abi = "cancelAuction(uint256)")]
    pub struct CancelAuctionCall {
        pub auction_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `codex`function with signature `codex()` and selector `[65, 119, 159, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "codex", abi = "codex()")]
    pub struct CodexCall;
    #[doc = "Container type for all input parameters for the `count`function with signature `count()` and selector `[6, 102, 26, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "count", abi = "count()")]
    pub struct CountCall;
    #[doc = "Container type for all input parameters for the `feeTip`function with signature `feeTip()` and selector `[148, 72, 48, 211]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "feeTip", abi = "feeTip()")]
    pub struct FeeTipCall;
    #[doc = "Container type for all input parameters for the `flatTip`function with signature `flatTip()` and selector `[151, 11, 72, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "flatTip", abi = "flatTip()")]
    pub struct FlatTipCall;
    #[doc = "Container type for all input parameters for the `getStatus`function with signature `getStatus(uint256)` and selector `[92, 98, 42, 14]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getStatus", abi = "getStatus(uint256)")]
    pub struct GetStatusCall {
        pub auction_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `init`function with signature `init(address,address)` and selector `[240, 154, 64, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "init", abi = "init(address,address)")]
    pub struct InitCall {
        pub vault: ethers::core::types::Address,
        pub collybus: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `limes`function with signature `limes()` and selector `[159, 146, 185, 110]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "limes", abi = "limes()")]
    pub struct LimesCall;
    #[doc = "Container type for all input parameters for the `list`function with signature `list()` and selector `[15, 86, 12, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "list", abi = "list()")]
    pub struct ListCall;
    #[doc = "Container type for all input parameters for the `redoAuction`function with signature `redoAuction(uint256,address)` and selector `[42, 46, 0, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "redoAuction", abi = "redoAuction(uint256,address)")]
    pub struct RedoAuctionCall {
        pub auction_id: ethers::core::types::U256,
        pub keeper: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setParam`function with signature `setParam(address,bytes32,address)` and selector `[41, 1, 162, 126]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setParam", abi = "setParam(address,bytes32,address)")]
    pub struct SetParam1Call {
        pub vault: ethers::core::types::Address,
        pub param: [u8; 32],
        pub data: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setParam`function with signature `setParam(bytes32,uint256)` and selector `[159, 48, 73, 10]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setParam", abi = "setParam(bytes32,uint256)")]
    pub struct SetParam0Call {
        pub param: [u8; 32],
        pub data: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setParam`function with signature `setParam(bytes32,address)` and selector `[237, 209, 176, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setParam", abi = "setParam(bytes32,address)")]
    pub struct SetParam3Call {
        pub param: [u8; 32],
        pub data: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setParam`function with signature `setParam(address,bytes32,uint256)` and selector `[246, 59, 15, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setParam", abi = "setParam(address,bytes32,uint256)")]
    pub struct SetParam4Call {
        pub vault: ethers::core::types::Address,
        pub param: [u8; 32],
        pub data: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `startAuction`function with signature `startAuction(uint256,uint256,address,uint256,address,address)` and selector `[21, 196, 134, 135]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "startAuction",
        abi = "startAuction(uint256,uint256,address,uint256,address,address)"
    )]
    pub struct StartAuctionCall {
        pub debt: ethers::core::types::U256,
        pub collateral_to_sell: ethers::core::types::U256,
        pub vault: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub keeper: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `stopped`function with signature `stopped()` and selector `[117, 241, 43, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "stopped", abi = "stopped()")]
    pub struct StoppedCall;
    #[doc = "Container type for all input parameters for the `takeCollateral`function with signature `takeCollateral(uint256,uint256,uint256,address,bytes)` and selector `[26, 208, 75, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "takeCollateral",
        abi = "takeCollateral(uint256,uint256,uint256,address,bytes)"
    )]
    pub struct TakeCollateralCall {
        pub auction_id: ethers::core::types::U256,
        pub collateral_amount: ethers::core::types::U256,
        pub max_price: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `updateAuctionDebtFloor`function with signature `updateAuctionDebtFloor(address)` and selector `[15, 70, 125, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "updateAuctionDebtFloor",
        abi = "updateAuctionDebtFloor(address)"
    )]
    pub struct UpdateAuctionDebtFloorCall {
        pub vault: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `vaults`function with signature `vaults(address)` and selector `[166, 34, 238, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "vaults", abi = "vaults(address)")]
    pub struct VaultsCall(pub ethers::core::types::Address);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum NoLossCollateralAuctionCalls {
        AnyCaller(AnyCallerCall),
        AnySig(AnySigCall),
        ActiveAuctions(ActiveAuctionsCall),
        Aer(AerCall),
        AllowCaller(AllowCallerCall),
        AuctionCounter(AuctionCounterCall),
        Auctions(AuctionsCall),
        BlockCaller(BlockCallerCall),
        CanCall(CanCallCall),
        CancelAuction(CancelAuctionCall),
        Codex(CodexCall),
        Count(CountCall),
        FeeTip(FeeTipCall),
        FlatTip(FlatTipCall),
        GetStatus(GetStatusCall),
        Init(InitCall),
        Limes(LimesCall),
        List(ListCall),
        RedoAuction(RedoAuctionCall),
        SetParam1(SetParam1Call),
        SetParam0(SetParam0Call),
        SetParam3(SetParam3Call),
        SetParam4(SetParam4Call),
        StartAuction(StartAuctionCall),
        Stopped(StoppedCall),
        TakeCollateral(TakeCollateralCall),
        UpdateAuctionDebtFloor(UpdateAuctionDebtFloorCall),
        Vaults(VaultsCall),
    }
    impl ethers::core::abi::AbiDecode for NoLossCollateralAuctionCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AnyCallerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::AnyCaller(decoded));
            }
            if let Ok(decoded) = <AnySigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::AnySig(decoded));
            }
            if let Ok(decoded) =
                <ActiveAuctionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::ActiveAuctions(decoded));
            }
            if let Ok(decoded) = <AerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(NoLossCollateralAuctionCalls::Aer(decoded));
            }
            if let Ok(decoded) =
                <AllowCallerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::AllowCaller(decoded));
            }
            if let Ok(decoded) =
                <AuctionCounterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::AuctionCounter(decoded));
            }
            if let Ok(decoded) =
                <AuctionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::Auctions(decoded));
            }
            if let Ok(decoded) =
                <BlockCallerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::BlockCaller(decoded));
            }
            if let Ok(decoded) =
                <CanCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::CanCall(decoded));
            }
            if let Ok(decoded) =
                <CancelAuctionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::CancelAuction(decoded));
            }
            if let Ok(decoded) = <CodexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::Codex(decoded));
            }
            if let Ok(decoded) = <CountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::Count(decoded));
            }
            if let Ok(decoded) = <FeeTipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::FeeTip(decoded));
            }
            if let Ok(decoded) =
                <FlatTipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::FlatTip(decoded));
            }
            if let Ok(decoded) =
                <GetStatusCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::GetStatus(decoded));
            }
            if let Ok(decoded) = <InitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(NoLossCollateralAuctionCalls::Init(decoded));
            }
            if let Ok(decoded) = <LimesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::Limes(decoded));
            }
            if let Ok(decoded) = <ListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(NoLossCollateralAuctionCalls::List(decoded));
            }
            if let Ok(decoded) =
                <RedoAuctionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::RedoAuction(decoded));
            }
            if let Ok(decoded) =
                <SetParam1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::SetParam1(decoded));
            }
            if let Ok(decoded) =
                <SetParam0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::SetParam0(decoded));
            }
            if let Ok(decoded) =
                <SetParam3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::SetParam3(decoded));
            }
            if let Ok(decoded) =
                <SetParam4Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::SetParam4(decoded));
            }
            if let Ok(decoded) =
                <StartAuctionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::StartAuction(decoded));
            }
            if let Ok(decoded) =
                <StoppedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::Stopped(decoded));
            }
            if let Ok(decoded) =
                <TakeCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::TakeCollateral(decoded));
            }
            if let Ok(decoded) =
                <UpdateAuctionDebtFloorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::UpdateAuctionDebtFloor(
                    decoded,
                ));
            }
            if let Ok(decoded) = <VaultsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(NoLossCollateralAuctionCalls::Vaults(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for NoLossCollateralAuctionCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                NoLossCollateralAuctionCalls::AnyCaller(element) => element.encode(),
                NoLossCollateralAuctionCalls::AnySig(element) => element.encode(),
                NoLossCollateralAuctionCalls::ActiveAuctions(element) => element.encode(),
                NoLossCollateralAuctionCalls::Aer(element) => element.encode(),
                NoLossCollateralAuctionCalls::AllowCaller(element) => element.encode(),
                NoLossCollateralAuctionCalls::AuctionCounter(element) => element.encode(),
                NoLossCollateralAuctionCalls::Auctions(element) => element.encode(),
                NoLossCollateralAuctionCalls::BlockCaller(element) => element.encode(),
                NoLossCollateralAuctionCalls::CanCall(element) => element.encode(),
                NoLossCollateralAuctionCalls::CancelAuction(element) => element.encode(),
                NoLossCollateralAuctionCalls::Codex(element) => element.encode(),
                NoLossCollateralAuctionCalls::Count(element) => element.encode(),
                NoLossCollateralAuctionCalls::FeeTip(element) => element.encode(),
                NoLossCollateralAuctionCalls::FlatTip(element) => element.encode(),
                NoLossCollateralAuctionCalls::GetStatus(element) => element.encode(),
                NoLossCollateralAuctionCalls::Init(element) => element.encode(),
                NoLossCollateralAuctionCalls::Limes(element) => element.encode(),
                NoLossCollateralAuctionCalls::List(element) => element.encode(),
                NoLossCollateralAuctionCalls::RedoAuction(element) => element.encode(),
                NoLossCollateralAuctionCalls::SetParam1(element) => element.encode(),
                NoLossCollateralAuctionCalls::SetParam0(element) => element.encode(),
                NoLossCollateralAuctionCalls::SetParam3(element) => element.encode(),
                NoLossCollateralAuctionCalls::SetParam4(element) => element.encode(),
                NoLossCollateralAuctionCalls::StartAuction(element) => element.encode(),
                NoLossCollateralAuctionCalls::Stopped(element) => element.encode(),
                NoLossCollateralAuctionCalls::TakeCollateral(element) => element.encode(),
                NoLossCollateralAuctionCalls::UpdateAuctionDebtFloor(element) => element.encode(),
                NoLossCollateralAuctionCalls::Vaults(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for NoLossCollateralAuctionCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                NoLossCollateralAuctionCalls::AnyCaller(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::AnySig(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::ActiveAuctions(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::Aer(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::AllowCaller(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::AuctionCounter(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::Auctions(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::BlockCaller(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::CanCall(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::CancelAuction(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::Codex(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::Count(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::FeeTip(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::FlatTip(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::GetStatus(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::Init(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::Limes(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::List(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::RedoAuction(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::SetParam1(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::SetParam0(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::SetParam3(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::SetParam4(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::StartAuction(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::Stopped(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::TakeCollateral(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::UpdateAuctionDebtFloor(element) => element.fmt(f),
                NoLossCollateralAuctionCalls::Vaults(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AnyCallerCall> for NoLossCollateralAuctionCalls {
        fn from(var: AnyCallerCall) -> Self {
            NoLossCollateralAuctionCalls::AnyCaller(var)
        }
    }
    impl ::std::convert::From<AnySigCall> for NoLossCollateralAuctionCalls {
        fn from(var: AnySigCall) -> Self {
            NoLossCollateralAuctionCalls::AnySig(var)
        }
    }
    impl ::std::convert::From<ActiveAuctionsCall> for NoLossCollateralAuctionCalls {
        fn from(var: ActiveAuctionsCall) -> Self {
            NoLossCollateralAuctionCalls::ActiveAuctions(var)
        }
    }
    impl ::std::convert::From<AerCall> for NoLossCollateralAuctionCalls {
        fn from(var: AerCall) -> Self {
            NoLossCollateralAuctionCalls::Aer(var)
        }
    }
    impl ::std::convert::From<AllowCallerCall> for NoLossCollateralAuctionCalls {
        fn from(var: AllowCallerCall) -> Self {
            NoLossCollateralAuctionCalls::AllowCaller(var)
        }
    }
    impl ::std::convert::From<AuctionCounterCall> for NoLossCollateralAuctionCalls {
        fn from(var: AuctionCounterCall) -> Self {
            NoLossCollateralAuctionCalls::AuctionCounter(var)
        }
    }
    impl ::std::convert::From<AuctionsCall> for NoLossCollateralAuctionCalls {
        fn from(var: AuctionsCall) -> Self {
            NoLossCollateralAuctionCalls::Auctions(var)
        }
    }
    impl ::std::convert::From<BlockCallerCall> for NoLossCollateralAuctionCalls {
        fn from(var: BlockCallerCall) -> Self {
            NoLossCollateralAuctionCalls::BlockCaller(var)
        }
    }
    impl ::std::convert::From<CanCallCall> for NoLossCollateralAuctionCalls {
        fn from(var: CanCallCall) -> Self {
            NoLossCollateralAuctionCalls::CanCall(var)
        }
    }
    impl ::std::convert::From<CancelAuctionCall> for NoLossCollateralAuctionCalls {
        fn from(var: CancelAuctionCall) -> Self {
            NoLossCollateralAuctionCalls::CancelAuction(var)
        }
    }
    impl ::std::convert::From<CodexCall> for NoLossCollateralAuctionCalls {
        fn from(var: CodexCall) -> Self {
            NoLossCollateralAuctionCalls::Codex(var)
        }
    }
    impl ::std::convert::From<CountCall> for NoLossCollateralAuctionCalls {
        fn from(var: CountCall) -> Self {
            NoLossCollateralAuctionCalls::Count(var)
        }
    }
    impl ::std::convert::From<FeeTipCall> for NoLossCollateralAuctionCalls {
        fn from(var: FeeTipCall) -> Self {
            NoLossCollateralAuctionCalls::FeeTip(var)
        }
    }
    impl ::std::convert::From<FlatTipCall> for NoLossCollateralAuctionCalls {
        fn from(var: FlatTipCall) -> Self {
            NoLossCollateralAuctionCalls::FlatTip(var)
        }
    }
    impl ::std::convert::From<GetStatusCall> for NoLossCollateralAuctionCalls {
        fn from(var: GetStatusCall) -> Self {
            NoLossCollateralAuctionCalls::GetStatus(var)
        }
    }
    impl ::std::convert::From<InitCall> for NoLossCollateralAuctionCalls {
        fn from(var: InitCall) -> Self {
            NoLossCollateralAuctionCalls::Init(var)
        }
    }
    impl ::std::convert::From<LimesCall> for NoLossCollateralAuctionCalls {
        fn from(var: LimesCall) -> Self {
            NoLossCollateralAuctionCalls::Limes(var)
        }
    }
    impl ::std::convert::From<ListCall> for NoLossCollateralAuctionCalls {
        fn from(var: ListCall) -> Self {
            NoLossCollateralAuctionCalls::List(var)
        }
    }
    impl ::std::convert::From<RedoAuctionCall> for NoLossCollateralAuctionCalls {
        fn from(var: RedoAuctionCall) -> Self {
            NoLossCollateralAuctionCalls::RedoAuction(var)
        }
    }
    impl ::std::convert::From<SetParam1Call> for NoLossCollateralAuctionCalls {
        fn from(var: SetParam1Call) -> Self {
            NoLossCollateralAuctionCalls::SetParam1(var)
        }
    }
    impl ::std::convert::From<SetParam0Call> for NoLossCollateralAuctionCalls {
        fn from(var: SetParam0Call) -> Self {
            NoLossCollateralAuctionCalls::SetParam0(var)
        }
    }
    impl ::std::convert::From<SetParam3Call> for NoLossCollateralAuctionCalls {
        fn from(var: SetParam3Call) -> Self {
            NoLossCollateralAuctionCalls::SetParam3(var)
        }
    }
    impl ::std::convert::From<SetParam4Call> for NoLossCollateralAuctionCalls {
        fn from(var: SetParam4Call) -> Self {
            NoLossCollateralAuctionCalls::SetParam4(var)
        }
    }
    impl ::std::convert::From<StartAuctionCall> for NoLossCollateralAuctionCalls {
        fn from(var: StartAuctionCall) -> Self {
            NoLossCollateralAuctionCalls::StartAuction(var)
        }
    }
    impl ::std::convert::From<StoppedCall> for NoLossCollateralAuctionCalls {
        fn from(var: StoppedCall) -> Self {
            NoLossCollateralAuctionCalls::Stopped(var)
        }
    }
    impl ::std::convert::From<TakeCollateralCall> for NoLossCollateralAuctionCalls {
        fn from(var: TakeCollateralCall) -> Self {
            NoLossCollateralAuctionCalls::TakeCollateral(var)
        }
    }
    impl ::std::convert::From<UpdateAuctionDebtFloorCall> for NoLossCollateralAuctionCalls {
        fn from(var: UpdateAuctionDebtFloorCall) -> Self {
            NoLossCollateralAuctionCalls::UpdateAuctionDebtFloor(var)
        }
    }
    impl ::std::convert::From<VaultsCall> for NoLossCollateralAuctionCalls {
        fn from(var: VaultsCall) -> Self {
            NoLossCollateralAuctionCalls::Vaults(var)
        }
    }
}
