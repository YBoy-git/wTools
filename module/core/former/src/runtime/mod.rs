
//!
//! Former - variation of builder pattern. Implementation of its runtime.
//!

/// Former of a fector.
mod vector;
/// Former of a hash map.
mod hash_map;
/// Former of a hash set.
mod hash_set;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::vector::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::hash_map::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::hash_set::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}