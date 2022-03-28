pub use ivault_mod::*;
#[allow(clippy::too_many_arguments)]
mod ivault_mod {
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
    #[doc = "IVault was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IVAULT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [],\n    \"name\": \"codex\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ICodex\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"collybus\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ICollybus\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"enter\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"exit\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"net\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"face\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"fairPrice\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"live\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"lock\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"maturity\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"param\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"data\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setParam\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"token\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"tokenScale\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"underlierScale\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"underlierToken\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"vaultType\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct IVault<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IVault<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IVault<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IVault))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IVault<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), IVAULT_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `codex` (0x41779f86) function"]
        pub fn codex(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([65, 119, 159, 134], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collybus` (0xe88c2f83) function"]
        pub fn collybus(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([232, 140, 47, 131], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enter` (0xd15d2e43) function"]
        pub fn enter(
            &self,
            token_id: ethers::core::types::U256,
            user: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 93, 46, 67], (token_id, user, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exit` (0xad894982) function"]
        pub fn exit(
            &self,
            token_id: ethers::core::types::U256,
            user: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 137, 73, 130], (token_id, user, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fairPrice` (0x06edbf77) function"]
        pub fn fair_price(
            &self,
            token_id: ethers::core::types::U256,
            net: bool,
            face: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([6, 237, 191, 119], (token_id, net, face))
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
        #[doc = "Calls the contract's `maturity` (0x644bd010) function"]
        pub fn maturity(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([100, 75, 208, 16], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setParam` (0xedd1b0be) function"]
        pub fn set_param(
            &self,
            param: [u8; 32],
            data: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 209, 176, 190], (param, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token` (0xfc0c546a) function"]
        pub fn token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenScale` (0x342c02b1) function"]
        pub fn token_scale(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([52, 44, 2, 177], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlierScale` (0x69a0125f) function"]
        pub fn underlier_scale(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([105, 160, 18, 95], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlierToken` (0x6db64e63) function"]
        pub fn underlier_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([109, 182, 78, 99], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vaultType` (0x4ac032be) function"]
        pub fn vault_type(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([74, 192, 50, 190], ())
                .expect("method not found (this should never happen)")
        }
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
    #[doc = "Container type for all input parameters for the `collybus`function with signature `collybus()` and selector `[232, 140, 47, 131]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "collybus", abi = "collybus()")]
    pub struct CollybusCall;
    #[doc = "Container type for all input parameters for the `enter`function with signature `enter(uint256,address,uint256)` and selector `[209, 93, 46, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "enter", abi = "enter(uint256,address,uint256)")]
    pub struct EnterCall {
        pub token_id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `exit`function with signature `exit(uint256,address,uint256)` and selector `[173, 137, 73, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "exit", abi = "exit(uint256,address,uint256)")]
    pub struct ExitCall {
        pub token_id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fairPrice`function with signature `fairPrice(uint256,bool,bool)` and selector `[6, 237, 191, 119]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fairPrice", abi = "fairPrice(uint256,bool,bool)")]
    pub struct FairPriceCall {
        pub token_id: ethers::core::types::U256,
        pub net: bool,
        pub face: bool,
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
    #[doc = "Container type for all input parameters for the `maturity`function with signature `maturity(uint256)` and selector `[100, 75, 208, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "maturity", abi = "maturity(uint256)")]
    pub struct MaturityCall {
        pub token_id: ethers::core::types::U256,
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
    pub struct SetParamCall {
        pub param: [u8; 32],
        pub data: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `token`function with signature `token()` and selector `[252, 12, 84, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    #[doc = "Container type for all input parameters for the `tokenScale`function with signature `tokenScale()` and selector `[52, 44, 2, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tokenScale", abi = "tokenScale()")]
    pub struct TokenScaleCall;
    #[doc = "Container type for all input parameters for the `underlierScale`function with signature `underlierScale()` and selector `[105, 160, 18, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "underlierScale", abi = "underlierScale()")]
    pub struct UnderlierScaleCall;
    #[doc = "Container type for all input parameters for the `underlierToken`function with signature `underlierToken()` and selector `[109, 182, 78, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "underlierToken", abi = "underlierToken()")]
    pub struct UnderlierTokenCall;
    #[doc = "Container type for all input parameters for the `vaultType`function with signature `vaultType()` and selector `[74, 192, 50, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "vaultType", abi = "vaultType()")]
    pub struct VaultTypeCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IVaultCalls {
        Codex(CodexCall),
        Collybus(CollybusCall),
        Enter(EnterCall),
        Exit(ExitCall),
        FairPrice(FairPriceCall),
        Live(LiveCall),
        Lock(LockCall),
        Maturity(MaturityCall),
        SetParam(SetParamCall),
        Token(TokenCall),
        TokenScale(TokenScaleCall),
        UnderlierScale(UnderlierScaleCall),
        UnderlierToken(UnderlierTokenCall),
        VaultType(VaultTypeCall),
    }
    impl ethers::core::abi::AbiDecode for IVaultCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <CodexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVaultCalls::Codex(decoded));
            }
            if let Ok(decoded) =
                <CollybusCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVaultCalls::Collybus(decoded));
            }
            if let Ok(decoded) = <EnterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVaultCalls::Enter(decoded));
            }
            if let Ok(decoded) = <ExitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IVaultCalls::Exit(decoded));
            }
            if let Ok(decoded) =
                <FairPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVaultCalls::FairPrice(decoded));
            }
            if let Ok(decoded) = <LiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IVaultCalls::Live(decoded));
            }
            if let Ok(decoded) = <LockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IVaultCalls::Lock(decoded));
            }
            if let Ok(decoded) =
                <MaturityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVaultCalls::Maturity(decoded));
            }
            if let Ok(decoded) =
                <SetParamCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVaultCalls::SetParam(decoded));
            }
            if let Ok(decoded) = <TokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVaultCalls::Token(decoded));
            }
            if let Ok(decoded) =
                <TokenScaleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVaultCalls::TokenScale(decoded));
            }
            if let Ok(decoded) =
                <UnderlierScaleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVaultCalls::UnderlierScale(decoded));
            }
            if let Ok(decoded) =
                <UnderlierTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVaultCalls::UnderlierToken(decoded));
            }
            if let Ok(decoded) =
                <VaultTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IVaultCalls::VaultType(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IVaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IVaultCalls::Codex(element) => element.encode(),
                IVaultCalls::Collybus(element) => element.encode(),
                IVaultCalls::Enter(element) => element.encode(),
                IVaultCalls::Exit(element) => element.encode(),
                IVaultCalls::FairPrice(element) => element.encode(),
                IVaultCalls::Live(element) => element.encode(),
                IVaultCalls::Lock(element) => element.encode(),
                IVaultCalls::Maturity(element) => element.encode(),
                IVaultCalls::SetParam(element) => element.encode(),
                IVaultCalls::Token(element) => element.encode(),
                IVaultCalls::TokenScale(element) => element.encode(),
                IVaultCalls::UnderlierScale(element) => element.encode(),
                IVaultCalls::UnderlierToken(element) => element.encode(),
                IVaultCalls::VaultType(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IVaultCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IVaultCalls::Codex(element) => element.fmt(f),
                IVaultCalls::Collybus(element) => element.fmt(f),
                IVaultCalls::Enter(element) => element.fmt(f),
                IVaultCalls::Exit(element) => element.fmt(f),
                IVaultCalls::FairPrice(element) => element.fmt(f),
                IVaultCalls::Live(element) => element.fmt(f),
                IVaultCalls::Lock(element) => element.fmt(f),
                IVaultCalls::Maturity(element) => element.fmt(f),
                IVaultCalls::SetParam(element) => element.fmt(f),
                IVaultCalls::Token(element) => element.fmt(f),
                IVaultCalls::TokenScale(element) => element.fmt(f),
                IVaultCalls::UnderlierScale(element) => element.fmt(f),
                IVaultCalls::UnderlierToken(element) => element.fmt(f),
                IVaultCalls::VaultType(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CodexCall> for IVaultCalls {
        fn from(var: CodexCall) -> Self {
            IVaultCalls::Codex(var)
        }
    }
    impl ::std::convert::From<CollybusCall> for IVaultCalls {
        fn from(var: CollybusCall) -> Self {
            IVaultCalls::Collybus(var)
        }
    }
    impl ::std::convert::From<EnterCall> for IVaultCalls {
        fn from(var: EnterCall) -> Self {
            IVaultCalls::Enter(var)
        }
    }
    impl ::std::convert::From<ExitCall> for IVaultCalls {
        fn from(var: ExitCall) -> Self {
            IVaultCalls::Exit(var)
        }
    }
    impl ::std::convert::From<FairPriceCall> for IVaultCalls {
        fn from(var: FairPriceCall) -> Self {
            IVaultCalls::FairPrice(var)
        }
    }
    impl ::std::convert::From<LiveCall> for IVaultCalls {
        fn from(var: LiveCall) -> Self {
            IVaultCalls::Live(var)
        }
    }
    impl ::std::convert::From<LockCall> for IVaultCalls {
        fn from(var: LockCall) -> Self {
            IVaultCalls::Lock(var)
        }
    }
    impl ::std::convert::From<MaturityCall> for IVaultCalls {
        fn from(var: MaturityCall) -> Self {
            IVaultCalls::Maturity(var)
        }
    }
    impl ::std::convert::From<SetParamCall> for IVaultCalls {
        fn from(var: SetParamCall) -> Self {
            IVaultCalls::SetParam(var)
        }
    }
    impl ::std::convert::From<TokenCall> for IVaultCalls {
        fn from(var: TokenCall) -> Self {
            IVaultCalls::Token(var)
        }
    }
    impl ::std::convert::From<TokenScaleCall> for IVaultCalls {
        fn from(var: TokenScaleCall) -> Self {
            IVaultCalls::TokenScale(var)
        }
    }
    impl ::std::convert::From<UnderlierScaleCall> for IVaultCalls {
        fn from(var: UnderlierScaleCall) -> Self {
            IVaultCalls::UnderlierScale(var)
        }
    }
    impl ::std::convert::From<UnderlierTokenCall> for IVaultCalls {
        fn from(var: UnderlierTokenCall) -> Self {
            IVaultCalls::UnderlierToken(var)
        }
    }
    impl ::std::convert::From<VaultTypeCall> for IVaultCalls {
        fn from(var: VaultTypeCall) -> Self {
            IVaultCalls::VaultType(var)
        }
    }
}
