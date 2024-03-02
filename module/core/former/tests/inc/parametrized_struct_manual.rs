#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, Default ) ]
pub struct Property< Name >
{
  name : Name,
  code : isize,
}

/// generated by new
impl< Name > Property< Name >
{
  #[ inline ]
  pub fn new< Code >( name : Name, code : Code ) -> Self
  where
    Name : core::convert::Into< Name >,
    Code : core::convert::Into< isize >,
  {
    Self { name : name.into(), code : code.into() }
  }
}

#[ derive( Debug, PartialEq ) ]
pub struct Command< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub name : String,
  pub properties : std::collections::HashMap< K, Property< K > >,
}

// generated by former
impl< K > Command< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{

  #[ inline( always ) ]
  pub fn former() -> CommandFormer< K >
  {
    CommandFormer::< K >::new()
  }

}

// generated by former
// #[ derive( Debug, Default ) ]
pub struct CommandFormer< K, Context = Command< K >, End = former::ReturnContainer >
where
  K : core::hash::Hash + std::cmp::Eq,
  End : former::ToSuperFormer< Command< K >, Context >,
{
  name : core::option::Option< String >,
  properties : core::option::Option< std::collections::HashMap< K, Property< K > > >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< End >,
}

// generated by former
impl< K, Context, End >
CommandFormer< K, Context, End >
where
  K : core::hash::Hash + std::cmp::Eq,
  End : former::ToSuperFormer< Command< K >, Context >,
{

  #[ inline( always ) ]
  fn form( mut self ) -> Command< K >
  {

    let name = if self.name.is_some()
    {
      self.name.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };

    let properties = if self.properties.is_some()
    {
      self.properties.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };

    Command
    {
      name,
      properties,
    }
  }

  #[ inline( always ) ]
  pub fn perform( self ) -> Command< K >
  {
    self.form()
  }

  #[ inline( always ) ]
  pub fn new() -> CommandFormer< K >
  {
    CommandFormer::< K >::begin
    (
      None,
      former::ReturnContainer,
    )
  }

  #[ inline( always ) ]
  pub fn begin
  (
    context :  core::option::Option< Context >,
    on_end : End,
  ) -> Self
  {
    Self
    {
      name : None,
      properties : None,
      context : context,
      on_end : Some( on_end ),
    }
  }

  /// Return former of your struct moving container there. Should be called after configuring the container.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Context
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    let container = self.form();
    on_end.call( container, context )
  }

  #[ inline( always ) ]
  pub fn name< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< String >,
  {
    debug_assert!( self.name.is_none() );
    self.name = Some( src.into() );
    self
  }

  #[ inline( always ) ]
  pub fn properties( mut self ) -> former::runtime::HashMapSubformer
  <
    K,
    Property< K >,
    std::collections::HashMap< K, Property< K > >,
    CommandFormer< K, Context, End >,
    impl former::ToSuperFormer< std::collections::HashMap< K, Property< K > >, Self >,
  >
  {
    let container = self.properties.take();
    let on_end = | container : std::collections::HashMap< K, Property< K > >, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      super_former.properties = Some( container );
      super_former
    };
    former::runtime::HashMapSubformer::begin( Some( self ), container, on_end )
  }

}

// ==

include!( "only_test/parametrized_struct.rs" );
