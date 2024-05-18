#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn from_named4()
{
  use the_module::{ Into1, VariadicFrom };

  #[ derive( Default, Debug, PartialEq, VariadicFrom ) ]
  struct Struct1
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  impl the_module::From_1< i32 > for Struct1
  {
    fn from_1( a : i32 ) -> Self { Self{ a, b : a, c : a, d : a } }
  }

  impl the_module::From_2< i32, i32 > for Struct1
  {
    fn from_2( a : i32, b : i32 ) -> Self { Self{ a, b, c : b, d : b } }
  }

  impl the_module::From_3< i32, i32, i32 > for Struct1
  {
    fn from_3( a : i32, b : i32, c : i32 ) -> Self { Self{ a, b, c, d : c } }
  }

  // 0

  let got : Struct1 = the_module::from!();
  let exp = Struct1{ a : 0, b : 0, c : 0, d : 0 };
  a_id!( got, exp );

  // 1

  let got : Struct1 = the_module::from!( 13 );
  let exp = Struct1{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( 13, ) );
  let exp = Struct1{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( ( 13, ), ) );
  let exp = Struct1{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : Struct1 = 13.to();
  let exp = Struct1{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : Struct1 = ( 13, ).to();
  let exp = Struct1{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  let got : Struct1 = ( ( 13, ), ).to();
  let exp = Struct1{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  // 2

  let got : Struct1 = the_module::from!( 0, 1 );
  let exp = Struct1{ a : 0, b : 1, c : 1, d : 1 };
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( 0, 1 ) );
  let exp = Struct1{ a : 0, b : 1, c : 1, d : 1 };
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( ( 0, 1 ), ) );
  let exp = Struct1{ a : 0, b : 1, c : 1, d : 1 };
  a_id!( got, exp );

  let got : Struct1 = ( 0, 1 ).to();
  let exp = Struct1{ a : 0, b : 1, c : 1, d : 1 };
  a_id!( got, exp );

  let got : Struct1 = ( ( 0, 1 ), ).to();
  let exp = Struct1{ a : 0, b : 1, c : 1, d : 1 };
  a_id!( got, exp );

  // 3

  let got : Struct1 = the_module::from!( 0, 1, 2 );
  let exp = Struct1{ a : 0, b : 1, c : 2, d : 2 };
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( 0, 1, 2 ) );
  let exp = Struct1{ a : 0, b : 1, c : 2, d : 2 };
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( ( 0, 1, 2 ), ) );
  let exp = Struct1{ a : 0, b : 1, c : 2, d : 2 };
  a_id!( got, exp );

  let got : Struct1 = ( 0, 1, 2 ).to();
  let exp = Struct1{ a : 0, b : 1, c : 2, d : 2 };
  a_id!( got, exp );

  let got : Struct1 = ( ( 0, 1, 2 ), ).to();
  let exp = Struct1{ a : 0, b : 1, c : 2, d : 2 };
  a_id!( got, exp );

}
