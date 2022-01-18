pub use collybus_mod::*;
#[allow(clippy::too_many_arguments)]
mod collybus_mod {
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
    #[doc = "Collybus was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static COLLYBUS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"Collybus__setParam_notLive\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"Collybus__setParam_unrecognizedParam\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"Collybus__updateDiscountRate_notLive\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"Collybus__updateSpot_notLive\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"Guarded__notGranted\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"Guarded__notRoot\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"x\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"y\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Math__add_overflow\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"x\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"y\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Math__mul_overflow\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"x\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"y\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Math__sub_overflow\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"prod1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"PRBMath__MulDivFixedPointOverflow\",\n    \"type\": \"error\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"sig\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"who\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"AllowCaller\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"sig\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"who\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"BlockCaller\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [],\n    \"name\": \"Lock\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"param\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"data\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"SetParam\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"rate\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"UpdateDiscountRate\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"spot\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"UpdateSpot\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ANY_SIG\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"sig\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"who\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowCaller\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"sig\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"who\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"blockCaller\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"sig\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"who\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"canCall\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"live\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"lock\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"rates\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"underlier\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maturity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"net\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"read\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"price\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"redemptionPrice\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"param\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"data\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setParam\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"param\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"data\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setParam\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"spots\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"rate\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"updateDiscountRate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"spot\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"updateSpot\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"vaults\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidationRatio\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct Collybus<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Collybus<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Collybus<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Collybus))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Collybus<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), COLLYBUS_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `ANY_SIG` (0x2936ff2b) function"]
        pub fn any_sig(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([41, 54, 255, 43], ())
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
        #[doc = "Calls the contract's `live` (0x957aa58c) function"]
        pub fn live(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([149, 122, 165, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lock` (0xf83d08ba) function"]
        pub fn lock(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 61, 8, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rates` (0xdd418ae2) function"]
        pub fn rates(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 65, 138, 226], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `read` (0xbd8edf6d) function"]
        pub fn read(
            &self,
            vault: ethers::core::types::Address,
            underlier: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
            maturity: ethers::core::types::U256,
            net: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [189, 142, 223, 109],
                    (vault, underlier, token_id, maturity, net),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redemptionPrice` (0xc5b748c0) function"]
        pub fn redemption_price(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([197, 183, 72, 192], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setParam` (0x9f30490a) function"]
        pub fn set_param(
            &self,
            param: [u8; 32],
            data: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 48, 73, 10], (param, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setParam` (0xf63b0f3d) function"]
        pub fn set_param_with_vault(
            &self,
            vault: ethers::core::types::Address,
            param: [u8; 32],
            data: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 59, 15, 61], (vault, param, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `spots` (0x21e7d9b3) function"]
        pub fn spots(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([33, 231, 217, 179], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateDiscountRate` (0xe90fd61e) function"]
        pub fn update_discount_rate(
            &self,
            token_id: ethers::core::types::U256,
            rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 15, 214, 30], (token_id, rate))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateSpot` (0x4320c627) function"]
        pub fn update_spot(
            &self,
            token: ethers::core::types::Address,
            spot: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([67, 32, 198, 39], (token, spot))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vaults` (0xa622ee7c) function"]
        pub fn vaults(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
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
        #[doc = "Gets the contract's `Lock` event"]
        pub fn lock_filter(&self) -> ethers::contract::builders::Event<M, LockFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetParam` event"]
        pub fn set_param_filter(&self) -> ethers::contract::builders::Event<M, SetParamFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdateDiscountRate` event"]
        pub fn update_discount_rate_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdateDiscountRateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdateSpot` event"]
        pub fn update_spot_filter(&self) -> ethers::contract::builders::Event<M, UpdateSpotFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, CollybusEvents> {
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
    #[ethevent(name = "Lock", abi = "Lock()")]
    pub struct LockFilter();
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "SetParam", abi = "SetParam(address,bytes32,uint256)")]
    pub struct SetParamFilter {
        #[ethevent(indexed)]
        pub vault: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub param: [u8; 32],
        pub data: ethers::core::types::U256,
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
        name = "UpdateDiscountRate",
        abi = "UpdateDiscountRate(uint256,uint256)"
    )]
    pub struct UpdateDiscountRateFilter {
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
        pub rate: ethers::core::types::U256,
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
    #[ethevent(name = "UpdateSpot", abi = "UpdateSpot(address,uint256)")]
    pub struct UpdateSpotFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub spot: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CollybusEvents {
        AllowCallerFilter(AllowCallerFilter),
        BlockCallerFilter(BlockCallerFilter),
        LockFilter(LockFilter),
        SetParamFilter(SetParamFilter),
        UpdateDiscountRateFilter(UpdateDiscountRateFilter),
        UpdateSpotFilter(UpdateSpotFilter),
    }
    impl ethers::contract::EthLogDecode for CollybusEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AllowCallerFilter::decode_log(log) {
                return Ok(CollybusEvents::AllowCallerFilter(decoded));
            }
            if let Ok(decoded) = BlockCallerFilter::decode_log(log) {
                return Ok(CollybusEvents::BlockCallerFilter(decoded));
            }
            if let Ok(decoded) = LockFilter::decode_log(log) {
                return Ok(CollybusEvents::LockFilter(decoded));
            }
            if let Ok(decoded) = SetParamFilter::decode_log(log) {
                return Ok(CollybusEvents::SetParamFilter(decoded));
            }
            if let Ok(decoded) = UpdateDiscountRateFilter::decode_log(log) {
                return Ok(CollybusEvents::UpdateDiscountRateFilter(decoded));
            }
            if let Ok(decoded) = UpdateSpotFilter::decode_log(log) {
                return Ok(CollybusEvents::UpdateSpotFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CollybusEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CollybusEvents::AllowCallerFilter(element) => element.fmt(f),
                CollybusEvents::BlockCallerFilter(element) => element.fmt(f),
                CollybusEvents::LockFilter(element) => element.fmt(f),
                CollybusEvents::SetParamFilter(element) => element.fmt(f),
                CollybusEvents::UpdateDiscountRateFilter(element) => element.fmt(f),
                CollybusEvents::UpdateSpotFilter(element) => element.fmt(f),
            }
        }
    }
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
    #[doc = "Container type for all input parameters for the `live`function with signature `live()` and selector `[149, 122, 165, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "live", abi = "live()")]
    pub struct LiveCall;
    #[doc = "Container type for all input parameters for the `lock`function with signature `lock()` and selector `[248, 61, 8, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lock", abi = "lock()")]
    pub struct LockCall;
    #[doc = "Container type for all input parameters for the `rates`function with signature `rates(uint256)` and selector `[221, 65, 138, 226]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rates", abi = "rates(uint256)")]
    pub struct RatesCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `read`function with signature `read(address,address,uint256,uint256,bool)` and selector `[189, 142, 223, 109]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "read", abi = "read(address,address,uint256,uint256,bool)")]
    pub struct ReadCall {
        pub vault: ethers::core::types::Address,
        pub underlier: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
        pub maturity: ethers::core::types::U256,
        pub net: bool,
    }
    #[doc = "Container type for all input parameters for the `redemptionPrice`function with signature `redemptionPrice()` and selector `[197, 183, 72, 192]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "redemptionPrice", abi = "redemptionPrice()")]
    pub struct RedemptionPriceCall;
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
    pub struct SetParamCall {
        pub param: [u8; 32],
        pub data: ethers::core::types::U256,
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
    pub struct SetParamWithVaultCall {
        pub vault: ethers::core::types::Address,
        pub param: [u8; 32],
        pub data: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `spots`function with signature `spots(address)` and selector `[33, 231, 217, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "spots", abi = "spots(address)")]
    pub struct SpotsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `updateDiscountRate`function with signature `updateDiscountRate(uint256,uint256)` and selector `[233, 15, 214, 30]`"]
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
        name = "updateDiscountRate",
        abi = "updateDiscountRate(uint256,uint256)"
    )]
    pub struct UpdateDiscountRateCall {
        pub token_id: ethers::core::types::U256,
        pub rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateSpot`function with signature `updateSpot(address,uint256)` and selector `[67, 32, 198, 39]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "updateSpot", abi = "updateSpot(address,uint256)")]
    pub struct UpdateSpotCall {
        pub token: ethers::core::types::Address,
        pub spot: ethers::core::types::U256,
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
    pub enum CollybusCalls {
        AnySig(AnySigCall),
        AllowCaller(AllowCallerCall),
        BlockCaller(BlockCallerCall),
        CanCall(CanCallCall),
        Live(LiveCall),
        Lock(LockCall),
        Rates(RatesCall),
        Read(ReadCall),
        RedemptionPrice(RedemptionPriceCall),
        SetParam(SetParamCall),
        SetParamWithVault(SetParamWithVaultCall),
        Spots(SpotsCall),
        UpdateDiscountRate(UpdateDiscountRateCall),
        UpdateSpot(UpdateSpotCall),
        Vaults(VaultsCall),
    }
    impl ethers::core::abi::AbiDecode for CollybusCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AnySigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollybusCalls::AnySig(decoded));
            }
            if let Ok(decoded) =
                <AllowCallerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollybusCalls::AllowCaller(decoded));
            }
            if let Ok(decoded) =
                <BlockCallerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollybusCalls::BlockCaller(decoded));
            }
            if let Ok(decoded) =
                <CanCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollybusCalls::CanCall(decoded));
            }
            if let Ok(decoded) = <LiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CollybusCalls::Live(decoded));
            }
            if let Ok(decoded) = <LockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CollybusCalls::Lock(decoded));
            }
            if let Ok(decoded) = <RatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollybusCalls::Rates(decoded));
            }
            if let Ok(decoded) = <ReadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CollybusCalls::Read(decoded));
            }
            if let Ok(decoded) =
                <RedemptionPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollybusCalls::RedemptionPrice(decoded));
            }
            if let Ok(decoded) =
                <SetParamCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollybusCalls::SetParam(decoded));
            }
            if let Ok(decoded) =
                <SetParamWithVaultCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollybusCalls::SetParamWithVault(decoded));
            }
            if let Ok(decoded) = <SpotsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollybusCalls::Spots(decoded));
            }
            if let Ok(decoded) =
                <UpdateDiscountRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollybusCalls::UpdateDiscountRate(decoded));
            }
            if let Ok(decoded) =
                <UpdateSpotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollybusCalls::UpdateSpot(decoded));
            }
            if let Ok(decoded) = <VaultsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollybusCalls::Vaults(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CollybusCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CollybusCalls::AnySig(element) => element.encode(),
                CollybusCalls::AllowCaller(element) => element.encode(),
                CollybusCalls::BlockCaller(element) => element.encode(),
                CollybusCalls::CanCall(element) => element.encode(),
                CollybusCalls::Live(element) => element.encode(),
                CollybusCalls::Lock(element) => element.encode(),
                CollybusCalls::Rates(element) => element.encode(),
                CollybusCalls::Read(element) => element.encode(),
                CollybusCalls::RedemptionPrice(element) => element.encode(),
                CollybusCalls::SetParam(element) => element.encode(),
                CollybusCalls::SetParamWithVault(element) => element.encode(),
                CollybusCalls::Spots(element) => element.encode(),
                CollybusCalls::UpdateDiscountRate(element) => element.encode(),
                CollybusCalls::UpdateSpot(element) => element.encode(),
                CollybusCalls::Vaults(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CollybusCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CollybusCalls::AnySig(element) => element.fmt(f),
                CollybusCalls::AllowCaller(element) => element.fmt(f),
                CollybusCalls::BlockCaller(element) => element.fmt(f),
                CollybusCalls::CanCall(element) => element.fmt(f),
                CollybusCalls::Live(element) => element.fmt(f),
                CollybusCalls::Lock(element) => element.fmt(f),
                CollybusCalls::Rates(element) => element.fmt(f),
                CollybusCalls::Read(element) => element.fmt(f),
                CollybusCalls::RedemptionPrice(element) => element.fmt(f),
                CollybusCalls::SetParam(element) => element.fmt(f),
                CollybusCalls::SetParamWithVault(element) => element.fmt(f),
                CollybusCalls::Spots(element) => element.fmt(f),
                CollybusCalls::UpdateDiscountRate(element) => element.fmt(f),
                CollybusCalls::UpdateSpot(element) => element.fmt(f),
                CollybusCalls::Vaults(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AnySigCall> for CollybusCalls {
        fn from(var: AnySigCall) -> Self {
            CollybusCalls::AnySig(var)
        }
    }
    impl ::std::convert::From<AllowCallerCall> for CollybusCalls {
        fn from(var: AllowCallerCall) -> Self {
            CollybusCalls::AllowCaller(var)
        }
    }
    impl ::std::convert::From<BlockCallerCall> for CollybusCalls {
        fn from(var: BlockCallerCall) -> Self {
            CollybusCalls::BlockCaller(var)
        }
    }
    impl ::std::convert::From<CanCallCall> for CollybusCalls {
        fn from(var: CanCallCall) -> Self {
            CollybusCalls::CanCall(var)
        }
    }
    impl ::std::convert::From<LiveCall> for CollybusCalls {
        fn from(var: LiveCall) -> Self {
            CollybusCalls::Live(var)
        }
    }
    impl ::std::convert::From<LockCall> for CollybusCalls {
        fn from(var: LockCall) -> Self {
            CollybusCalls::Lock(var)
        }
    }
    impl ::std::convert::From<RatesCall> for CollybusCalls {
        fn from(var: RatesCall) -> Self {
            CollybusCalls::Rates(var)
        }
    }
    impl ::std::convert::From<ReadCall> for CollybusCalls {
        fn from(var: ReadCall) -> Self {
            CollybusCalls::Read(var)
        }
    }
    impl ::std::convert::From<RedemptionPriceCall> for CollybusCalls {
        fn from(var: RedemptionPriceCall) -> Self {
            CollybusCalls::RedemptionPrice(var)
        }
    }
    impl ::std::convert::From<SetParamCall> for CollybusCalls {
        fn from(var: SetParamCall) -> Self {
            CollybusCalls::SetParam(var)
        }
    }
    impl ::std::convert::From<SetParamWithVaultCall> for CollybusCalls {
        fn from(var: SetParamWithVaultCall) -> Self {
            CollybusCalls::SetParamWithVault(var)
        }
    }
    impl ::std::convert::From<SpotsCall> for CollybusCalls {
        fn from(var: SpotsCall) -> Self {
            CollybusCalls::Spots(var)
        }
    }
    impl ::std::convert::From<UpdateDiscountRateCall> for CollybusCalls {
        fn from(var: UpdateDiscountRateCall) -> Self {
            CollybusCalls::UpdateDiscountRate(var)
        }
    }
    impl ::std::convert::From<UpdateSpotCall> for CollybusCalls {
        fn from(var: UpdateSpotCall) -> Self {
            CollybusCalls::UpdateSpot(var)
        }
    }
    impl ::std::convert::From<VaultsCall> for CollybusCalls {
        fn from(var: VaultsCall) -> Self {
            CollybusCalls::Vaults(var)
        }
    }
}
