
mod bindings{include!{concat!{env!{"OUT_DIR"}, "/gl_bindings.rs"}}}
pub use bindings::{*, types::*};

