// mod cauldron;
// pub use cauldron::*;

// mod witch;
// pub use witch::*;

// mod flashliquidator;
// pub use flashliquidator::*;

mod codex;
pub use codex::*;

mod collybus;
pub use collybus::*;

mod imulticall2;
pub use imulticall2::CallData as IMulticall2Call;
pub use imulticall2::ResultData as IMulticall2Result;
pub use imulticall2::*;

// keccak256(VaultAddr, TokenId, UserAddr)
pub type PositionIdType = [u8; 32];
// VaultAddr
pub type VaultIdType = [u8; 20];
// TokenId
pub type TokenIdType = [u8; 32];
// TokenAdrr
pub type TokenType = [u8; 20];
// pub type BaseIdType = [u8; 6];
