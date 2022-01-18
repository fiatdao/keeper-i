mod codex;
pub use codex::*;

mod collybus;
pub use collybus::*;

mod imulticall2;
pub use imulticall2::CallData as IMulticall2Call;
pub use imulticall2::ResultData as IMulticall2Result;
pub use imulticall2::*;

/// Uniquely identifies a position (keccak256 of the vault address, tokenId and user / owner address)
pub type PositionIdType = [u8; 32];
/// Uniquely identifies a vault (address of the vault)
pub type VaultIdType = [u8; 20];
/// Uniquely identifies a ERC721 / ERC1155 style token (tokenId)
pub type TokenIdType = [u8; 32];

// // Uniquely identifies a ERC20 style token (address of the token)
// pub type TokenType = [u8; 20];
// Uniquely identifies a discount rate used for computing the fair price of a positions collateral
pub type RateIdType = [u8; 32];
// Uniquely identifies a the spot price of a positions underlier used for computing the fair price it
pub type SpotIdType = [u8; 20];

/// Uniquely identifies a Update / Event used by the watcher to sort incoming events
pub type UpdateIdType = [u8; 32];
