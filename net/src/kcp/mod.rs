mod error;
mod kcp;

/// The `KCP` prelude
pub mod prelude {
    pub use super::{get_conv, Kcp, KCP_OVERHEAD};
}

pub use error::Error;
pub use kcp::{get_conv, get_sn, set_conv, Kcp, KCP_OVERHEAD};

/// KCP result
pub type KcpResult<T> = Result<T, Error>;
