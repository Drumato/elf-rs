//! ELF symbol utilities.

pub use elf32::*;
pub use elf64::*;
pub use symbol_bind::*;
pub use symbol_type::*;
pub use symbol_visibility::*;

mod elf32;
mod elf64;
mod symbol_bind;
mod symbol_type;
mod symbol_visibility;

/* definitions for st_info(bind) */
/// Local Symbol
pub const STB_LOCAL: u8 = 0;
/// Global Symbol
pub const STB_GLOBAL: u8 = 1;
