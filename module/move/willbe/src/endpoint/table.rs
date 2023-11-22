mod private
{
  use std::
  { 
    fs, 
    path::PathBuf
  };
  use std::io::
  { 
    Read, 
    Seek,
    SeekFrom 
  };
  use std::io::Write;
  use cargo_metadata::
  {
    Dependency,
    DependencyKind,
    MetadataCommand,
    Package
  };
  use wca::wtools::Itertools;
  use convert_case::Case;
  use convert_case::Casing;
  use std::fs::OpenOptions;
  use std::path::Path;
  use std::str::FromStr;

  use error_tools::for_app::
  {
    Error,
    Result,
    anyhow,
    bail,
  };
  use crate::package::functions;
  use crate::package::functions::FilterMapOptions;
  use walkdir::WalkDir;
  use toml::Value;


  lazy_static::lazy_static!
  {
    static ref TAG_TEMPLATE: regex::bytes::Regex = regex::bytes::Regex::new( r#"<!--\{ generate.healthtable\( '(\w+/\w+)' \) \} -->"# ).unwrap();
    static ref CLOUSE_TAG: regex::bytes::Regex = regex::bytes::Regex::new( r#"<!--\{ generate\.healthtable\.end \} -->"# ).unwrap();
  }



  enum Stability
  {
    Stable,
    Experimental,
    Deprecated,
  }

  impl FromStr for Stability
  {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
      match s
      {
        "Stable" => Ok( Stability::Stable ),
        "Experimental" => Ok( Stability::Experimental ),
        "Deprecated" => Ok( Stability::Deprecated ),
        _ => Err( err!( "Fail to parse stability" ) ),
      }
    }
  }

  fn get_stable_status( directories: Vec<String>, dir: &Path ) -> Result< Vec< ( String, Stability ) > >
  {
    let mut results = Vec::new();

    for directory in directories
    {
      for entry in WalkDir::new( dir.join(directory ) )
      {
        let entry = entry?;
        if entry.file_name() == "Cargo.toml"
        {
          let contents = fs::read_to_string( entry.path() )?;
          let value = contents.parse::< Value >()?;
          let stable_status = value
          .get( "health.table" )
          .and_then( | package | package.get( "stable_status" ) )
          .and_then( Value::as_str )
          .and_then( | s | s.parse::< Stability >().ok() );
          results.push( ( entry.path().display().to_string(), stable_status.unwrap_or( Stability::Stable ) ) );
        }
      }
    }

    Ok(results)
  }


  struct TableParameters
  {
    core_url: String,
    user_and_repo: String,
    branches: Vec< String >,
  }

  /// Create health table in README.md file
  ///
  /// The location and filling of tables is defined by a tag, for example record:
  /// ```md
  /// <!--{ generate.healthtable( 'module/core' ) } -->
  /// <!--{ generate.healthtable.end } -->
  /// ```
  /// will mean that at this place the table with modules located in the directory module/core will be generated.
  /// The tags do not disappear after generation.
  /// Anything between the opening and closing tag will be destroyed.
  pub fn table_create() -> Result< () >
  {
    let cargo_metadata = MetadataCommand::new().no_deps().exec()?;
    let workspace_root = workspace_root( &cargo_metadata )?;
    let read_me_path = readme_path(&workspace_root).ok_or_else( || anyhow!( "Fail to find README.md" ) )?;
    let mut file = OpenOptions::new()
    .read( true )
    .write( true )
    .open( &read_me_path )?;

    let params = TableParameters{ core_url: "https://github.com/Wandalen/wTools/".into(), user_and_repo: "Wandalen/wTools".into(), branches: vec![ "alpha".to_string(), "master".to_string() ] };

    let mut contents = Vec::new();

    file.read_to_end( &mut contents )?;
    let mut buffer = vec![];
    let mut tags_closures = vec![];
    let mut tables = vec![];
    let open_caps = TAG_TEMPLATE.captures_iter( &*contents );
    let close_caps = CLOUSE_TAG.captures_iter( &*contents );
    // iterate by regex matches and generate table content for each dir which taken from open-tag
    for ( open_captures, close_captures ) in open_caps.zip( close_caps )
    {
      for captures in open_captures.iter().zip( close_captures.iter() )
      {
        if let ( Some( open ), Some( close ) ) = captures
        {
          let module_path = PathBuf::from_str
          (
            std::str::from_utf8
            (
              TAG_TEMPLATE.captures( open.as_bytes() )
              .ok_or( anyhow!( "Fail to parse tag" ) )?
              .get( 1 )
              .ok_or( anyhow!( "Fail to parse group" ) )?
              .as_bytes()
            )?
          )?;
          let directory_names = directory_names( workspace_root.join(module_path.clone()), &cargo_metadata.packages );
          let directory_names_and_stability = get_stable_status( directory_names, &workspace_root )?;
          let table = table_prepare( directory_names_and_stability,&workspace_root.join(module_path.clone()), &params );
          tables.push( table );
          tags_closures.push( ( open.end(), close.start() ) );
        }
      }
    }
    let mut start: usize = 0;
    for ( ( end_of_start_tag, start_of_end_tag ), con ) in tags_closures.iter().zip( tables.iter() )
    {
      copy_range_to_target( &*contents, &mut buffer, start, *end_of_start_tag )?;
      copy_range_to_target( con.as_bytes(), &mut buffer, 0,con.len() - 1 )?;
      start = *start_of_end_tag;
    }
    copy_range_to_target( &*contents,&mut buffer,start,contents.len() - 1 )?;

    file.set_len( 0 )?;
    file.seek( SeekFrom::Start( 0 ) )?;

    file.write_all( &buffer )?;

    Ok( () )
  }

  fn get_repo_url_and_branches( path: &Path ) -> Result< ( String, Vec< String > ) >
  {
    let cargo_toml_path = path.join( "Cargo.toml" );
    if !cargo_toml_path.exists()
    {
      bail!( "Cannot find Cargo.toml" )
    }
    {
      let contents = fs::read_to_string( cargo_toml_path )?;
      let value = contents.parse::< Value >()?;

      let repo_url = value
      .get( "package" )
      .and_then( | package | package.get( "repo_url" ) )
      .and_then( Value::as_str )
      .map( String::from );

      let branches = value
      .get( "package" )
      .and_then( | package | package.get( "branches" ) )
      .and_then( Value::as_array )
      .map
      (
        | array |
        array
        .iter()
        .filter_map( Value::as_str )
        .map( String::from )
        .collect()
      );

    }
  }


  fn directory_names( path: PathBuf, packages: &[ Package ] ) -> Vec< String >
  {
    let path_clone = path.clone();
    let module_package_filter: Option< Box< dyn Fn( &Package) -> bool > > = Some
    (
      Box::new
      (
        move | p |
        {
          p.publish.is_none() && p.manifest_path.starts_with(&path)
        }
      )
    );
    let module_dependency_filter: Option< Box< dyn Fn( &Package, &Dependency) -> bool > > = Some
    (
      Box::new
        (
          move | _, d |
          {
            d.path.is_some() && d.kind != DependencyKind::Development && d.path.as_ref().unwrap().starts_with( &path_clone )
          }
        )
    );
    let module_packages_map = functions::packages_filter_map
    (
      packages,
      FilterMapOptions{ package_filter: module_package_filter, dependency_filter: module_dependency_filter },
    );
    let module_graph = functions::graph_build( &module_packages_map);
    functions::toposort( module_graph )
  }

  fn table_prepare( modules: Vec< ( String, Stability ) >, dir: &Path, parameters: &TableParameters ) -> String
  {
    let table_header = generate_table_header(&parameters);
    let stability = generate_stability( &parameters );

    let table_content = modules
    .iter()
    .map
    (
      | ( module_name, module_stability ) |
      {
        let cell_module = format!("[{}](./{}/{})", &module_name, &dir.display(), &module_name);
        let cell_branch = generate_branch_cells(module_stability, module_name );
        let cell_docs = format!("[![docs.rs](https://raster.shields.io/static/v1?label=&message=docs&color=eee)](https://docs.rs/{})", &module_name );
        let cell_sample = format!("[![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F{}_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20{}_trivial_sample/{})", &module_name, &module_name, parameters.core_url );
        format!("| {} | {} | {} | {} | {} |", cell_module, stability, cell_branch, cell_docs, cell_sample)
      }
    )
    .join( "\n" );
    format!( "{table_header}\n{table_content}\n" )
  }

  fn generate_stability( stability: Stability ) -> String
  {
    match stability
    {
      Stability::Experimental => "[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)".into(),
      Stability::Stable => "[![stable](https://raster.shields.io/static/v1?label=&message=stable&color=green)](https://github.com/emersion/stability-badges#stable)".into(),
      Stabiluity::Deprecated => "[![deprecated](https://raster.shields.io/static/v1?label=&message=deprecated&color=grey)](https://github.com/emersion/stability-badges#deprecated)".into()
    }
  }

  fn generate_table_header( table_parameters: &TableParameters ) -> String
  {
    let mut table_header = String::from( "| Module | Stability |" );

    for branch in &table_parameters.branches
    {
      table_header.push_str(&format!(" {} |", branch.to_case( Case::Title) ) );
    }

    table_header.push_str( " Docs | Online |\n|--------|-----------|" );

    for _ in &table_parameters.branches
    {
      table_header.push_str( "--------|" );
    }

    table_header.push_str( ":----:|:------:|" );

    table_header
  }

  fn generate_branch_cells( table_parameters: &TableParameters, module_name: &String ) -> String
  {
    table_parameters
    .branches
    .iter()
    .map
    (
      | b |
      format!( "[![rust-status](https://img.shields.io/github/actions/workflow/status/{}/Module{}Push.yml?label=&branch={b})](https://{}/actions/workflows/Module{}Push.yml)", table_parameters.user_and_repo, &module_name.to_case( Case::Pascal ), table_parameters.core_url, &module_name.to_case( Case::Pascal ) )
    )
    .collect::< Vec< String > >()
    .join( " | ")
  }

  fn workspace_root( metadata: &cargo_metadata::Metadata ) -> Result< PathBuf >
  {
    Ok( metadata.workspace_root.clone().into_std_path_buf() )
  }

  fn copy_range_to_target< T: Clone >( source: &[T], target: &mut Vec< T >, from: usize, to: usize ) -> Result< () >
  {
    if from < source.len() && to < source.len() && from <= to
    {
      target.extend_from_slice( &source[ from..= to ] );
      return Ok( () )
    }
    else
    {
      bail!( "Incorrect indexes" )
    }
  }

  /// Searches for a README file in specific subdirectories of the given directory path.
  ///
  /// This function attempts to find a README file in the following subdirectories: ".github",
  /// the root directory, and "./docs". It returns the path to the first found README file, or
  /// `None` if no README file is found in any of these locations.
  fn readme_path( dir_path : &Path ) -> Option< PathBuf >
  {
    if let Some( path )  = readme_in_dir_find(&dir_path.join( ".github" ) )
    {
      Some( path )
    }
    else if let Some( path )  = readme_in_dir_find( dir_path )
    {
      Some( path )
    }
    else if let Some( path )  = readme_in_dir_find( &dir_path.join( "docs" ) )
    {
      Some( path )
    }
    else
    {
      None
    }
  }


  /// Searches for a file named "readme.md" in the specified directory path.
  ///
  /// Given a directory path, this function searches for a file named "readme.md" in the specified
  /// directory.
  fn readme_in_dir_find( path: &Path ) -> Option< PathBuf >
  {
    fs::read_dir( path )
    .ok()?
    .filter_map( Result::ok )
    .filter( | p | p.path().is_file() )
    .filter_map( | f |
    {
      let l_f = f.file_name().to_ascii_lowercase();
      if l_f == "readme.md"
      {
        return Some( f.file_name() )
      }
      None
    })
    .max()
    .map( PathBuf::from )
  }
}

crate::mod_interface!
{
  /// Create Table.
  prelude use table_create;
}
