// xxx : complete
use super::*;

// let ca = Aggregator::former()
// .parameter1( "val" )
// .command( "echo" )
//   .name( "prints all subjects and properties" )
//   .subject( "Subject", wca::Type::String, true )
//   .property( "property", "simple property", wca::Type::String, true )
//   .routine( f1 )
//   .end()
// .command( "exit" )
//   .name( "just exit" )
//   .routine( || exit() )
//   .end()
// .perform()
// ;
// ca.execute( input ).unwrap();

#[ derive( Debug, PartialEq, Default ) ]
pub struct Property< Name >
{
  name : Name,
  description : String,
  code : isize,
}

/// generated by new
impl< Name > Property< Name >
{
  #[ inline ]
  pub fn new< Description, Code >( name : Name, description : Description, code : Code ) -> Self
  where
    Name : core::convert::Into< Name >,
    Description : core::convert::Into< String >,
    Code : core::convert::Into< isize >,
  {
    Self { name : name.into(), description : description.into(), code : code.into() }
  }
}

#[ derive( Debug, PartialEq ) ]
pub struct Command< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub name : String,
  pub subject : String,
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
  subject : core::option::Option< String >,
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

    let subject = if self.subject.is_some()
    {
      self.subject.take().unwrap()
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
      subject,
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
      subject : None,
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
  pub fn subject< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< String >,
  {
    debug_assert!( self.subject.is_none() );
    self.subject = Some( src.into() );
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

impl< K, Context, End >
CommandFormer< K, Context, End >
where
  K : core::hash::Hash + std::cmp::Eq,
  End : former::ToSuperFormer< Command< K >, Context >,
{

  /// Inserts a key-value pair into the map. Make a new container if it was not made so far.
  #[ inline( always ) ]
  pub fn property< Name, Description, Code >
  ( mut self, name : Name, description : Description, code : Code ) -> Self
  where
    Name : core::convert::Into< K > + Clone,
    Description : core::convert::Into< String >,
    Code : core::convert::Into< isize >,
  {
    if self.properties.is_none()
    {
      self.properties = core::option::Option::Some( Default::default() );
    }
    if let core::option::Option::Some( ref mut properties ) = self.properties
    {
      let property = Property
      {
        name : name.clone().into(),
        description : description.into(),
        code : code.into(),
      };
      properties.insert( name.into(), property );
    }
    self
  }

}

// == aggregator

#[ derive( Debug, PartialEq ) ]
pub struct Aggregator< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub parameter1 : String,
  pub commands : std::collections::HashMap< String, Command< K > >,
}

// generated by former
impl< K > Aggregator< K >
where
  K : core::hash::Hash + std::cmp::Eq,
{

  #[ inline( always ) ]
  pub fn former() -> AggregatorFormer< K >
  {
    AggregatorFormer::< K >::new()
  }

}

// generated by former
// #[ derive( Debug, Default ) ]
pub struct AggregatorFormer< K, Context = Aggregator< K >, End = former::ReturnContainer >
where
  K : core::hash::Hash + std::cmp::Eq,
  End : former::ToSuperFormer< Aggregator< K >, Context >,
{
  parameter1 : core::option::Option< String >,
  commands : core::option::Option< std::collections::HashMap< String, Command< K > > >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< End >,
}

// generated by former
impl< K, Context, End >
AggregatorFormer< K, Context, End >
where
  K : core::hash::Hash + std::cmp::Eq,
  End : former::ToSuperFormer< Aggregator< K >, Context >,
{

  #[ inline( always ) ]
  fn form( mut self ) -> Aggregator< K >
  {

    let parameter1 = if self.parameter1.is_some()
    {
      self.parameter1.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };

    let commands = if self.commands.is_some()
    {
      self.commands.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };

    Aggregator
    {
      parameter1,
      commands,
    }
  }

  #[ inline( always ) ]
  pub fn perform( self ) -> Aggregator< K >
  {
    self.form()
  }

  #[ inline( always ) ]
  pub fn new() -> AggregatorFormer< K >
  {
    AggregatorFormer::< K >::begin
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
      parameter1 : None,
      commands : None,
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
  pub fn parameter1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< String >,
  {
    debug_assert!( self.parameter1.is_none() );
    self.parameter1 = Some( src.into() );
    self
  }

  #[ inline( always ) ]
  pub fn commands( mut self ) -> former::runtime::HashMapSubformer
  <
    String,
    Command< K >,
    std::collections::HashMap< String, Command< K > >,
    AggregatorFormer< K, Context, End >,
    // impl Fn( std::collections::HashMap< String, Command< K > >, Self ) -> Self,
    impl former::ToSuperFormer< std::collections::HashMap< String, Command< K > >, Self >,
  >
  {
    let container = self.commands.take();
    let on_end = | container : std::collections::HashMap< String, Command< K > >, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      super_former.commands = Some( container );
      super_former
    };
    former::runtime::HashMapSubformer::begin( Some( self ), container, on_end )
  }

  #[ inline( always ) ]
  pub fn command( self, name : String ) -> CommandFormer< K, Self, impl former::ToSuperFormer< Command< K >, Self > >
  where
    K : core::hash::Hash + std::cmp::Eq,
  {
    let on_end = | command : Command< K >, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      if let Some( ref mut commands ) = super_former.commands
      {
        commands.insert( command.name.clone(), command );
      }
      else
      {
        let mut commands : std::collections::HashMap< String, Command< K > > = Default::default();
        commands.insert( command.name.clone(), command );
        super_former.commands = Some( commands );
      }
      super_former
    };
    let former = CommandFormer::begin( Some( self ), on_end );
    former.name( name )
  }

}

// ==

include!( "only_test/subformer_basic.rs" );
