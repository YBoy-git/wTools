use super::*;

//

tests_impls!
{
  fn basic()
  {
    let parser = Parser::former().form();

    // only one command
    a_id!
    (
      Program { commands : vec!
      [
        ParsedCommand
        {
          name : "command".into(),
          subjects : vec![],
          properties : HashMap::new(),
        },
      ]},
      parser.program( ".command" ).unwrap()
    );

    a_id!
    (
      Program { commands : vec!
      [
        ParsedCommand
        {
          name : "command1".into(),
          subjects : vec![],
          properties : HashMap::new(),
        },
        ParsedCommand
        {
          name : "command2".into(),
          subjects : vec![],
          properties : HashMap::new(),
        },
        ParsedCommand
        {
          name : "command3".into(),
          subjects : vec![],
          properties : HashMap::new(),
        }
      ]},
      parser.program( ".command1 .command2 .command3" ).unwrap()
    );
  }
}

//

tests_index!
{
  basic,
}
