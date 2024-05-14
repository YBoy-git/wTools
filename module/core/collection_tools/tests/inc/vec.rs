use super::*;

#[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
#[ test ]
fn reexport()
{

  let mut vec1 : the_module::Vec< i32 > = the_module::Vec::new();
  vec1.push( 1 );
  vec1.push( 2 );
  let got = vec1.first().unwrap().clone();
  assert_eq!( got, 1 );
  let got = vec1.last().unwrap().clone();
  assert_eq!( got, 2 );

  let mut vec2 : the_module::DynArray< i32 > = the_module::DynArray::new();
  vec2.push( 1 );
  vec2.push( 2 );
  let got = vec2.first().unwrap().clone();
  assert_eq!( got, 1 );
  let got = vec2.last().unwrap().clone();
  assert_eq!( got, 2 );

  assert_eq!( vec1, vec2 );

}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn constructor()
{

  // test.case( "empty" );
  let got : the_module::Vec< i32 > = the_module::vec!{};
  let exp = the_module::Vec::< i32 >::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::vec!{ 3, 13 };
  let mut exp = the_module::Vec::new();
  exp.push( 3 );
  exp.push( 13 );
  assert_eq!( got, exp );

}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_constructor()
{

  // test.case( "empty" );
  let got : the_module::Vec< i32 > = the_module::into_vec!{};
  let exp = the_module::Vec::< i32 >::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got : the_module::Vec< i32 > = the_module::into_vec!{ 3, 13 };
  let mut exp = the_module::Vec::new();
  exp.push( 3 );
  exp.push( 13 );
  assert_eq!( got, exp );

}

// qqq : implement similar test for all containers
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn vec_iters()
{

  struct MyContainer
  {
    entries : Vec< i32 >,
  }

  impl IntoIterator for MyContainer
  {
    type Item = i32;
    type IntoIter = the_module::vec::IntoIter< i32 >;
    // qqq : should work -- works

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.into_iter() // Create an iterator from the internal HashSet.
    }
  }

  impl< 'a > IntoIterator for &'a MyContainer
  {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter< 'a, i32 >;
    // type IntoIter = the_module::vec::Iter< 'a, i32 >;
    // qqq : should work

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter() // Borrow the elements via an iterator.
    }
  }

  impl< 'a > IntoIterator for &'a mut MyContainer
  {
    type Item = &'a mut i32;
    type IntoIter = std::slice::IterMut< 'a, i32 >;
    // type IntoIter = the_module::vec::IterMut< 'a, i32 >;
    // qqq : should work

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter_mut()
    }
  }

  let instance = MyContainer { entries : vec![ 1, 2, 3 ] };
  let got : Vec< _ > = ( &instance ).into_iter().cloned().collect();
  let exp = vec![ 1, 2, 3 ];
  a_id!( got, exp );

  let instance = MyContainer { entries : vec![ 1, 2, 3 ] };
  let got : Vec< _ > = instance.into_iter().collect();
  let exp = vec![ 1, 2, 3 ];
  a_id!( got, exp );

}