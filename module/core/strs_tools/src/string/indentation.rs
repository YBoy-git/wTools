/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Add indentation to each line.
  ///

  pub fn indentation< Prefix, Src, Postfix >( prefix : Prefix, src : Src, postfix : Postfix ) -> String
  where
    Prefix : AsRef< str >,
    Src : AsRef< str >,
    Postfix : AsRef< str >,
  {
    let prefix = prefix.as_ref();
    let postfix = postfix.as_ref();
    let splits = src
    .as_ref()
    .split( '\n' )
    ;

    splits
    .map( | e | prefix.to_owned() + e + postfix )
    .enumerate()
    // intersperse is unstable
    // .intersperse( '\n' )
    .fold( String::new(), | mut a, b |
    {
      a.push_str( if b.0 > 0 { "\n" } else { "" } );
      a.push_str( &b.1 );
      a
    })
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Parented namespace of the module.
pub mod orphan
{
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    indentation,
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
}