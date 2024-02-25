const ASSETS_PATH : &str = "tests/assets";

use assert_fs::prelude::*;
use crate::TheModule::endpoint::{ self };

mod header_create_test
{ 
  use std::io::Read;
  use willbe::path::AbsolutePath;
  
  use super::*;
  
  fn arrange( source : &str ) -> assert_fs::TempDir 
  { 
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );
    
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( source ), &[ "**" ] ).unwrap();
    
    temp 
  }
  
  #[ test ]
  fn with_full_config() 
  { 
    // Arrange
    let temp = arrange( "single_module" );
    
    let expected = "<!--{ generate.main_header.start() }-->\n[![test_branch](https://img.shields.io/github/actions/workflow/status/Username/test/StandardRustScheduled.yml?branch=master&label=test_branch&logo=github)](https://github.com/Username/test/actions/workflows/StandardRustStatus.yml)\n[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)\n[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Ftest_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20test_trivial_sample/https://github.com/Username/test)\n[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/test)\n<!--{ generate.main_header.end }-->";
    
    // Act
    _ = endpoint::generate_main_header( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
    
    let mut file = std::fs::File::open( temp.path().join( "Readme.md" ) ).unwrap();
    
    let mut actual = String::new();
    
    _ = file.read_to_string( &mut actual ).unwrap();
    
    // Assert
    assert_eq!( expected, actual ); 
  }
  
  #[ test ]
  fn without_fool_config() 
  { 
    // Arrange
    let temp = arrange( "single_module_without_master_branch_and_discord" );
    
    let expected = "<!--{ generate.main_header.start() }-->\n[![master](https://img.shields.io/github/actions/workflow/status/Username/test/StandardRustScheduled.yml?branch=master&label=master&logo=github)](https://github.com/Username/test/actions/workflows/StandardRustStatus.yml)\n[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Ftest_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20test_trivial_sample/https://github.com/Username/test)\n[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/test)\n<!--{ generate.main_header.end }-->";
    
    // Act
    _ = endpoint::generate_main_header( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
    
    let mut file = std::fs::File::open( temp.path().join( "Readme.md" ) ).unwrap();
    
    let mut actual = String::new();
    
    _ = file.read_to_string( &mut actual ).unwrap();
    
    // Assert
    assert_eq!( expected, actual ); 
  }
  
  #[ test ]
  fn idempotency() 
  { 
    // Arrange
    let temp = arrange( "single_module" );
    
    let expected = "<!--{ generate.main_header.start() }-->\n[![test_branch](https://img.shields.io/github/actions/workflow/status/Username/test/StandardRustScheduled.yml?branch=master&label=test_branch&logo=github)](https://github.com/Username/test/actions/workflows/StandardRustStatus.yml)\n[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)\n[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Ftest_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20test_trivial_sample/https://github.com/Username/test)\n[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/test)\n<!--{ generate.main_header.end }-->";
    
    // Act
    _ = endpoint::generate_main_header( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
    _ = endpoint::generate_main_header( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
    _ = endpoint::generate_main_header( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
    _ = endpoint::generate_main_header( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
    
    let mut file = std::fs::File::open( temp.path().join( "Readme.md" ) ).unwrap();
    
    let mut actual = String::new();
    
    _ = file.read_to_string( &mut actual ).unwrap();
    
    // Assert
    assert_eq!( expected, actual ); 
  }
  
  #[ test ]
  #[ should_panic ]
  fn without_needed_config() 
  { 
    // Arrange
    let temp = arrange( "variadic_tag_configurations" ); 
    // Act
    _ = endpoint::generate_main_header( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap(); 
  } 
}