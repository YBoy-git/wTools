#![ allow( dead_code ) ]

use super::*;

/// Child
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child
{
  name : String,
  is_mandatory : bool,
}

/// Parent

#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  #[ container( setter = false ) ]
  // #[ scalar_setter( false ) ]
  children : Vec< Child >,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  // xxx
  #[ inline( always ) ]
  pub fn children( self ) -> &'static str
  {
    r#"
    Scalar setter `children` should not be generated by default if subform is used.
    It can only be generated if req
    "#
  }

  #[ inline( always ) ]
  pub fn children2( self ) -> former::ContainerSubformer::
  <
    Child,
    former::VectorDefinition< Child, Self, Self, ParentFormerAssignChildrenEnd< Definition >, >
  >
  {
    self._children_assign::< _ >()
  }

}

#[ test ]
fn container()
{

  let got = Parent::former()
  .children2()
    .add( Child::former().name( "a" ).form() )
    .add( Child::former().name( "b" ).form() )
    .end()
  .form();

  let children = vec!
  [
    Child { name : "a".to_string(), is_mandatory : false },
    Child { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
