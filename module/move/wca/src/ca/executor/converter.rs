pub( crate ) mod private
{
  use crate::*;
  use former::Former;
  use std::collections::HashMap;
  use wtools::{ error::Result, err };

  /// This is the struct that provides a way to convert a `VerifiedCommand` to an `ExecutableCommand_`.
  ///
  /// The conversion is done by looking up the `Routine` associated with the command in a HashMap of routines.
  ///
  /// ```
  /// # use wca::{ Command, Type, VerifiedCommand, ExecutorConverter, Routine };
  /// # use std::collections::HashMap;
  /// # fn main() -> Result< (), Box< dyn std::error::Error > > {
  /// let executor_converter = ExecutorConverter::former()
  /// .routine( "command", Routine::new( |( args, props )| Ok( () ) ) )
  /// .form();
  ///
  /// let grammar_command = VerifiedCommand
  /// {
  ///   phrase : "command".to_string(),
  ///   subjects : vec![],
  ///   properties : HashMap::new(),
  /// };
  ///
  /// let executable_command = executor_converter.to_command( grammar_command )?;
  /// # Ok( () ) }
  /// ```
  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  pub struct ExecutorConverter
  {
    pub( crate ) routines : HashMap< String, Routine >,
  }

  impl ExecutorConverterFormer
  {
    /// Inserts routine to a routine dictionary
    pub fn routine< S >( mut self, phrase : S, routine : Routine ) -> Self
    where
      S : Into< String >,
      Routine : Into< Routine >,
    {
      let mut routines = self.storage.routines.unwrap_or_default();

      routines.insert( phrase.into(), routine );

      self.storage.routines = Some( routines );
      self
    }
  }

  impl ExecutorConverter
  {
    /// Converts raw program to executable
    pub fn to_program( &self, raw_program : Program< VerifiedCommand > ) -> Result< Program< ExecutableCommand_ > >
    {
      let commands = raw_program.commands
      .into_iter()
      .map( | n | self.to_command( n ) )
      .collect::< Result< Vec< ExecutableCommand_ > > >()?;

      Ok( Program { commands } )
    }

    /// Converts raw command to executable
    pub fn to_command( &self, command : VerifiedCommand ) -> Result< ExecutableCommand_ >
    {
      self.routines
      .get( &command.phrase )
      .ok_or_else( || err!( "Can not found routine for command `{}`", command.phrase ) )
      .map
      (
        | routine | ExecutableCommand_
        {
          subjects : command.subjects,
          properties : command.properties,
          routine : routine.clone(),
        }
      )
    }
  }
}

//

crate::mod_interface!
{
  exposed use ExecutorConverter;
}
