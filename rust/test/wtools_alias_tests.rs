#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]
#![ allow( non_snake_case ) ]

// #![ allow( non_snake_case ) ]
#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

use wTools as TheModule;
use test_tools::*;

#[ path = "./mod.rs" ]
mod tests;