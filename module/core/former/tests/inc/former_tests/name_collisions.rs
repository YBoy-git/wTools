#[ allow( unused_imports ) ]
use super::*;

#[ allow( dead_code ) ]
type Option = ();
#[ allow( dead_code ) ]
type Some = ();
#[ allow( dead_code ) ]
type None = ();
#[ allow( dead_code ) ]
type Result = ();
#[ allow( dead_code ) ]
type Ok = ();
#[ allow( dead_code ) ]
type Err = ();
#[ allow( dead_code ) ]
type Box = ();
#[ allow( dead_code ) ]
type Default = ();
#[ allow( dead_code ) ]
type HashSet = ();
#[ allow( dead_code ) ]
type HashMap = ();

#[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ derive( Debug, PartialEq ) ]
// #[ debug ]
pub struct Struct1
{
  vec_1 : Vec< String >,
  hashmap_1 : std::collections::HashMap< String, String >,
  hashset_1 : std::collections::HashSet< String >,
}

//

include!( "./only_test/containers_without_subformer.rs" );
