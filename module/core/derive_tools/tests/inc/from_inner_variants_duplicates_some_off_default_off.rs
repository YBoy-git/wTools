#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, the_module::From ) ]
#[ from( off ) ]
// #[ debug ]
pub enum GetData
{
  Nothing,
  Nothing2,
  FromString( String ),
  #[ from( on ) ]
  FromString2( String ),
  FromPair( String, String ),
  #[ from( on ) ]
  FromPair2( String, String ),
  FromBin( &'static [ u8 ] ),
  Nothing3,
}

// == begin of generated

// == end of generated

#[ test ]
fn variant_from()
{

  let got : GetData = From::from( &b"abc"[ .. ] );
  let exp = GetData::FromBin( b"abc" );
  a_id!( got, exp );

  let got : GetData = From::from( "abc".to_string() );
  let exp = GetData::FromString2( "abc".to_string() );
  a_id!( got, exp );

  let got : GetData = From::from( ( "a".to_string(), "b".to_string() ) );
  let exp = GetData::FromPair2( "a".to_string(), "b".to_string() );
  a_id!( got, exp );

}
