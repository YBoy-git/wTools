use std::path::Path;
use super::*;
use assert_fs::prelude::*;
use TheModule::endpoint::{ self, list::* };
use willbe::CrateDir;
use willbe::path::AbsolutePath;

const ASSETS_PATH : &str = "tests/assets";

//

fn crate_dir( path : &Path ) -> CrateDir
{
  let absolut = AbsolutePath::try_from( path ).unwrap();
  CrateDir::try_from( absolut ).unwrap()
}

// a -> b -> c
mod chain_of_three_packages
{
  use super::*;

  fn arrange() -> assert_fs::TempDir
  {
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "chain_of_packages" ), &[ "**" ] ).unwrap();

    temp
  }

  #[ test ]
  fn tree_format_for_single_package()
  {
    // Arrange
    let temp = arrange();
    let args = ListArgs::former()
    .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
    .format( ListFormat::Tree )
    .dependency_sources([ DependencySource::Local ])
    .dependency_categories([ DependencyCategory::Primary ])
    .form();

    // Act
    let output = endpoint::list( args ).unwrap();

    // Assert
    let ListReport::Tree { graph, names } = &output else { panic!( "Expected `Tree` format, but found another" ) };

    assert_eq!( 3, graph.node_count() );
    assert_eq!( 2, graph.edge_count() );
    assert_eq!( 1, names.len() );
    assert_eq!( Some( "_chain_of_packages_a" ), graph.node_weight( names[ 0 ] ).map( | x | x.as_str() ) );
  }

  #[ test ]
  fn list_format_for_single_package()
  {
    // Arrange
    let temp = arrange();
    let args = ListArgs::former()
    .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
    .format( ListFormat::Topological )
    .dependency_sources([ DependencySource::Local ])
    .dependency_categories([ DependencyCategory::Primary ])
    .form();

    // Act
    let output = endpoint::list( args ).unwrap();

    // Assert
    let ListReport::List( names ) = &output else { panic!("Expected `Topological` format, but found another") };

    assert_eq!( &[ "_chain_of_packages_c".to_string(), "_chain_of_packages_b".to_string(), "_chain_of_packages_a".to_string() ], names.as_slice() );
  }

  #[ test ]
  fn list_format_for_whole_workspace()
  {
    // Arrange
    let temp = arrange();
    let args = ListArgs::former()
    .path_to_manifest( crate_dir( &temp ) )
    .format( ListFormat::Topological )
    .dependency_sources([ DependencySource::Local ])
    .dependency_categories([ DependencyCategory::Primary ])
    .form();

    // Act
    let output = endpoint::list( args ).unwrap();

    // Assert
    let ListReport::List( names ) = &output else { panic!( "Expected `Topological` format, but found another" ) };

    assert_eq!( &[ "_chain_of_packages_c".to_string(), "_chain_of_packages_b".to_string(), "_chain_of_packages_a".to_string() ], names.as_slice() );
  }
}

// a -> ( remote, b )
mod package_with_remote_dependency
{
  use super::*;

  fn arrange() -> assert_fs::TempDir
  {
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "package_with_remote_dependency" ), &[ "**" ] ).unwrap();

    temp
  }

  #[ test ]
  fn tree_format_for_single_package()
  {
    // Arrange
    let temp = arrange();
    let args = ListArgs::former()
    .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
    .format( ListFormat::Tree )
    .dependency_sources([ DependencySource::Local, DependencySource::Remote ])
    .dependency_categories([ DependencyCategory::Primary ])
    .form();

    // Act
    let output = endpoint::list( args ).unwrap();

    // Assert
    let ListReport::Tree { graph, names } = &output else { panic!( "Expected `Tree` format, but found another" ) };

    assert_eq!( 3, graph.node_count() );
    assert_eq!( 2, graph.edge_count() );
    assert_eq!( 1, names.len() );
    assert_eq!( Some( "_package_with_remote_dep_a" ), graph.node_weight( names[ 0 ] ).map( | x | x.as_str() ) );
  }

  #[ test ]
  fn list_format_for_single_package()
  {
    // Arrange
    let temp = arrange();
    let args = ListArgs::former()
    .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
    .format( ListFormat::Topological )
    .dependency_sources([ DependencySource::Local, DependencySource::Remote ])
    .dependency_categories([ DependencyCategory::Primary ])
    .form();

    // Act
    let output = endpoint::list( args ).unwrap();

    // Assert
    let ListReport::List( names ) = &output else { panic!( "Expected `Topological` format, but found another" ) };

    assert_eq!( 3, names.len() );
    // `a` must be last
    assert_eq!( "_package_with_remote_dep_a", &names[ 2 ] );
    // can be in any order
    assert!( ( "_package_with_remote_dep_b" == &names[ 0 ] && "foo" == &names[ 1 ] ) || ( "_package_with_remote_dep_b" == &names[ 1 ] && "foo" == &names[ 0 ] ) );
  }

  #[ test ]
  fn only_local_dependency_filter()
  {
    // Arrange
    let temp = arrange();
    let args = ListArgs::former()
    .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
    .format( ListFormat::Topological )
    .dependency_sources([ DependencySource::Local ])
    .dependency_categories([ DependencyCategory::Primary ])
    .form();

    // Act
    let output = endpoint::list( args ).unwrap();

    // Assert
    let ListReport::List( names ) = &output else { panic!( "Expected `Topological` format, but found another" ) };

    assert_eq!( &[ "_package_with_remote_dep_b".to_string(), "_package_with_remote_dep_a".to_string() ], names.as_slice() );
  }
}

// a -> b -> a
mod workspace_with_cyclic_dependency
{
  use super::*;

  #[ test ]
  fn can_not_show_list_with_cyclic_dependencies()
  {
    // Arrange
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "workspace_with_cyclic_dependency" ), &[ "**" ] ).unwrap();

    let args = ListArgs::former()
    .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
    .format( ListFormat::Tree )
    .dependency_sources([ DependencySource::Local, DependencySource::Remote ])
    .dependency_categories([ DependencyCategory::Primary, DependencyCategory::Dev ])
    .form();

    // Act
    let output = endpoint::list( args );

    // Assert

    // can not process topological sorting for cyclic dependencies
    assert!( output.is_err() );
  }
}
