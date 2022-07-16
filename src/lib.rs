pub mod func;
pub mod macros;
pub mod traits;
pub use glam;

pub mod prelude {
    pub use super::*;
    pub use crate::func::*;
    pub use crate::traits::*;
    pub use glam::*;
}
