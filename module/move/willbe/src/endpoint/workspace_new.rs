mod private
{
	use std::fs;
	use std::io::Write;
	use std::path::Path;
	use error_tools::for_app::bail;
	use error_tools::Result;

	/// Creates workspace template
	pub fn workspace_new( path: &Path ) -> Result< () >
	{
		if fs::read_dir( path )?.count() != 0
		{
			bail!("Directory should be empty")
		}

		create_dir( path, ".cargo" )?;
		create_file( &path.join( ".cargo" ), "config.toml", include_str!("../../files/template/.cargo/config.toml") )?;

		create_dir( path, ".circleci" )?;
		create_file( &path.join( ".circleci" ), "config.yml", include_str!("../../files/template/.circleci/config.yml") )?;

		create_dir( path, ".github" )?;
		create_dir( &path.join( ".github" ), "workflow" )?;

		create_dir( path, "assets" )?;

		create_dir( path, "docs" )?;

		create_file( path, "Readme.md", include_str!("../../files/template/Readme.md" ) )?;

		create_file( path, ".gitattributes", include_str!("../../files/template/.gitattributes" ) )?;

		create_file( path, ".gitignore", include_str!("../../files/template/.gitignore" ) )?;

		create_file( path, ".gitpod.yml", include_str!("../../files/template/.gitpod.yml" ) )?;

		create_file( path, "Cargo.toml", include_str!("../../files/template/Cargo.toml" ) )?;

		create_file( path, "Makefile", include_str!("../../files/template/Makefile" ) )?;

		create_dir( path, "module" )?;

		create_dir( &path.join( "module" ), "example_module" )?;

		create_file( &path.join( "module" ).join( "example_module" ), "Cargo.toml", include_str!("../../files/template/module/example_module/Cargo.toml" ) )?;

		create_file( &path.join( "module" ).join( "example_module" ), "Readme.md", include_str!("../../files/template/module/example_module/Readme.md" ) )?;

		create_dir( &path.join( "module" ).join( "example_module" ), "examples" )?;

		create_dir( &path.join( "module" ).join( "example_module" ), "src" )?;

		create_dir( &path.join( "module" ).join( "example_module" ), "tests" )?;

		create_file( &path.join( "module" ).join( "example_module" ).join( "examples" ), "example_module_trivial_sample.rs", include_str!("../../files/template/module/example_module/examples/example_module_trivial_sample.rs" ) )?;

		create_file( &path.join( "module" ).join( "example_module" ).join( "src" ), "lib.rs", include_str!("../../files/template/module/example_module/src/lib.rs" ) )?;

		create_file( &path.join( "module" ).join( "example_module" ).join( "tests" ), "hello_test.rs", include_str!("../../files/template/module/example_module/tests/hello_test.rs" ) )?;

		Ok(())
	}

	fn create_dir( path: &Path, name: &str ) -> Result< () >
	{
		fs::create_dir( path.join( name ) )?;
		Ok( () )
	}

	fn create_file( path: &Path, name: &str, content: &str ) -> Result< () >
	{
		let mut file = fs::File::create( path.join( name ) )?;
		file.write_all( content.as_bytes() )?;
		Ok( () )
	}


}

crate::mod_interface!
{
  prelude use workspace_new;
}