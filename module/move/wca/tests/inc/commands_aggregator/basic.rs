use super::*;

//

tests_impls!
{
  fn simple()
  {
    let ca = CommandsAggregator::former()
    .command( "command" )
      .hint( "hint" )
      .long_hint( "long_hint" )
      .routine( || println!( "Command" ) )
      .end()
    .perform();

    a_id!( (), ca.perform( ".command" ).unwrap() ); // Parse -> Validate -> Execute
  }

  fn with_only_general_help()
  {
    let ca = CommandsAggregator::former()
    .command( "command" )
      .hint( "hint" )
      .long_hint( "long_hint" )
      .routine( || println!( "Command" ) )
      .end()
    .help_variants([ HelpVariants::General ])
    .perform();

    a_id!( (), ca.perform( ".help" ).unwrap() ); // raw string -> GrammarProgram -> ExecutableProgram -> execute

    a_true!( ca.perform( ".help command" ).is_err() );

    a_true!( ca.perform( ".help.command" ).is_err() );
  }

  fn custom_parser()
  {
    let parser = Parser::former()
    .command_prefix( '-' )
    .form();

    let ca = CommandsAggregator::former()
    .parser( parser )
    .command( "command" )
      .hint( "hint" )
      .long_hint( "long_hint" )
      .routine( || println!( "command" ) )
      .end()
    .perform();

    a_id!( (), ca.perform( "-command" ).unwrap() );
  }

  fn dot_command()
  {
    let ca = CommandsAggregator::former()
    .command( "cmd.first" )
      .hint( "hint" )
      .long_hint( "long_hint" )
      .routine( || println!( "Command" ) )
      .end()
    .command( "cmd.second" )
      .hint( "hint" )
      .long_hint( "long_hint" )
      .routine( || println!( "Command2" ) )
      .end()
    .perform();

    a_id!( (), ca.perform( "." ).unwrap() );
    a_id!( (), ca.perform( ".cmd." ).unwrap() );

    a_true!( ca.perform( ".c." ).is_err() );
  }

  fn error_types()
  {
    let ca = CommandsAggregator::former()
    .command( "command" )
      .hint( "hint" )
      .long_hint( "long_hint" )
      .routine( || println!( "command" ) )
      .end()
    .command( "command_with_execution_error" )
      .hint( "hint" )
      .long_hint( "long_hint" )
      .routine( || { println!( "command" ); Err( "runtime error" ) } )
      .end()
    .perform();

    a_true!( ca.perform( ".command" ).is_ok() );
    // Expect execution error
    a_true!
    (
      matches!
      (
        ca.perform( ".command_with_execution_error" ),
        Err( Error::Execution( _ ) )
      ),
      "Unexpected error type, expected Error::Execution."
    );
    // Expect ValidationError::Verifier
    a_true!
    (
      matches!
      (
        ca.perform( ".help.help.help" ),
        Err( Error::Validation( ValidationError::Verifier( _ ) ) )
      ),
      "Unexpected validation error type, expected ValidationError::Verifier."
    );
    // Expect ValidationError::Parser
    a_true!
    (
      matches!
      (
        ca.perform( "command" ),
        Err( Error::Validation( ValidationError::Parser { .. } ) )
      ),
      "Unexpected validation error type, expected ValidationError::Parser."
    );
  }

  // tests bug fix when passing a subject with a colon character
  // example: passing the path to a directory with a colon in its name
  fn path_subject_with_colon()
  {
    let ca = CommandsAggregator::former()
    .command( "command" )
      .hint( "hint" )
      .long_hint( "long_hint" )
      .subject().hint( "A path to directory." ).kind( Type::Path ).optional( true ).end()
      .routine( || println!( "hello" ) )
      .end()
    .perform();

    let command = r#".command "./path:to_dir" "#;

    a_id!( (), ca.perform( command ).unwrap() );

    let wrong_command = r#".command ./path:to_dir "#;

    a_true!
    (
      matches!
      (
        ca.perform( wrong_command ),
        Err( Error::Validation( ValidationError::Parser { .. } ) )
      ),
      "It is a sentence that can not be parsed: `/path:to_dir`"
    );
  }

  fn string_subject_with_colon()
  {
    let dictionary = &the_module::Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject().hint( "Any string." ).kind( Type::String ).optional( true ).end()
      .property( "nightly" ).hint( "Some property." ).kind( Type::String ).optional( true ).end()
      .routine( || println!( "hello" ) )
      .form()
    )
    .perform();
    let parser = Parser::former().form();
    use the_module::CommandParser;
    let grammar = the_module::Verifier;
    let executor = the_module::Executor::former().form();

    let command = r#".command qwe:rty nightly:true "#;

    let raw_command = parser.command( command ).unwrap();
    let grammar_command = grammar.to_command( dictionary, raw_command ).unwrap();

    a_id!( grammar_command.subjects, vec![ the_module::Value::String( "qwe:rty".into() ) ] );

    a_id!( (), executor.command( dictionary, grammar_command ).unwrap() );
  }

  fn no_prop_subject_with_colon()
  {
    let dictionary = &the_module::Dictionary::former()
    .command
    (
      the_module::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject().hint( "Any string." ).kind( Type::String ).optional( true ).end()
      .routine( || println!( "hello" ) )
      .form()
    )
    .form();

    let command = r#".command qwe:rty"#;

    let parser = Parser::former().form();
    use the_module::CommandParser;
    let grammar = the_module::Verifier;
    let executor = the_module::Executor::former().form();

    let raw_command = parser.command( command ).unwrap();
    let grammar_command = grammar.to_command( dictionary, raw_command ).unwrap();

    a_id!( grammar_command.subjects, vec![ the_module::Value::String( "qwe:rty".into() ) ] );

    a_id!( (), executor.command( dictionary, grammar_command ).unwrap() );
  }

  fn optional_prop_subject_with_colon()
  {
    let dictionary = &the_module::Dictionary::former()
    .command
    (
      the_module::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject().hint( "Any string." ).kind( Type::String ).optional( true ).end()
      .property( "nightly" ).hint( "Some property." ).kind( Type::String ).optional( true ).end()
      .routine( || println!( "hello" ) )
      .form()
    )
    .form();

    let command = r#".command qwe:rty"#;

    let parser = Parser::former().form();
    use the_module::CommandParser;
    let grammar = the_module::Verifier;
    let executor = the_module::Executor::former().form();

    let raw_command = parser.command( command ).unwrap();
    let grammar_command = grammar.to_command( dictionary, raw_command ).unwrap();

    a_id!( grammar_command.subjects, vec![ the_module::Value::String("qwe:rty".into()) ] );

    a_id!( (), executor.command( dictionary, grammar_command ).unwrap() );
  }
}

//

tests_index!
{
  simple,
  with_only_general_help,
  custom_parser,
  dot_command,
  error_types,
  path_subject_with_colon,
  string_subject_with_colon,
  no_prop_subject_with_colon,
  optional_prop_subject_with_colon,
}
