/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// Context.
  #[ clone_dyn ]
  pub trait ChangeInterface
  where
    Self :
      fmt::Debug +
    ,
  {

    /// Add change to queue of events.
    fn add_to< C : ChangerInterface >( self, changer : &mut C ) -> &mut C
    where
      Self : Sized + 'static,
    {
      changer.change_add( self )
    }

  }

  //

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
  };
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::
  {
    prelude::*,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::
  {
    private::ChangeInterface,
  };
}