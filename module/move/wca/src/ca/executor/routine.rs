pub( crate ) mod private
{
  use crate::*;

  use std::collections::HashMap;
  use wtools::error::Result;

  use std::{ fmt::Formatter, rc::Rc };
  use wtools::anyhow::anyhow;

  /// Command Args
  ///
  /// Used to contain subjects of a command and allow the user to retrieve them in comfortable way.
  ///
  /// # Example:
  ///
  /// ```
  /// use wca::{ Args, Value };
  ///
  /// let args = Args( vec![ Value::String( "Hello, World!".to_string() ) ] );
  ///
  /// let first_arg : &str = args.get_owned( 0 ).unwrap();
  /// assert_eq!( "Hello, World!", first_arg );
  ///
  /// let first_arg : &str = args[ 0 ].clone().into();
  /// assert_eq!( "Hello, World!", first_arg );
  /// ```
  ///
  /// ## Use case
  /// ```
  /// # use wca::{ Routine, Args };
  /// let routine = Routine::new
  /// (
  ///   |( args, _ ) : ( Args, _ )|
  ///   {
  ///     let first_arg : i32 = args.get_owned( 0 ).unwrap();
  ///
  ///     Ok( () )
  ///   }
  /// );
  /// ```
  #[ derive( Debug ) ]
  pub struct Args( pub Vec< Value > );

  impl Args
  {
    /// Returns owned casted value by its index
    ///
    /// ```
    /// # use wca::{ Args, Value };
    ///
    /// let args = Args( vec![ Value::String( "Hello, World!".to_string() ) ] );
    ///
    /// let first_arg : &str = args.get_owned( 0 ).unwrap();
    /// assert_eq!( "Hello, World!", first_arg );
    ///
    /// let first_arg : &str = args[ 0 ].clone().into();
    /// assert_eq!( "Hello, World!", first_arg );
    /// ```
    pub fn get_owned< T : From< Value > >( &self, index : usize ) -> Option< T >
    {
      self.0.get( index ).map( | arg | arg.to_owned().into() )
    }
  }

  impl core::ops::Deref for Args
  {
    type Target = Vec< Value >;
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  /// Command Properties
  ///
  /// Used to contain properties of a command and allow the user to retrieve them in comfortable way.
  ///
  /// # Example:
  ///
  /// ```
  /// use wca::{ Props, Value };
  ///
  /// let props = Props( [ ( "hello".to_string(), Value::String( "World!".to_string() ) ) ].into() );
  /// let hello_prop : &str = props.get_owned( "hello" ).unwrap();
  ///
  /// assert_eq!( "World!", hello_prop );
  /// ```
  ///
  /// ## Use case
  /// ```
  /// # use wca::{ Routine, Props };
  /// let routine = Routine::new
  /// (
  ///   |( _, props ) : ( _, Props )|
  ///   {
  ///     let key_option : i32 = props.get_owned( "key" ).unwrap();
  ///
  ///     Ok( () )
  ///   }
  /// );
  /// ```
  #[ derive( Debug ) ]
  pub struct Props( pub HashMap< String, Value > );

  impl Props
  {
    /// Returns owned casted value by its key
    ///
    /// ```
    /// # use wca::{ Props, Value };
    ///
    /// let props = Props( [ ( "hello".to_string(), Value::String( "World!".to_string() ) ) ].into() );
    /// let hello_prop : &str = props.get_owned( "hello" ).unwrap();
    ///
    /// assert_eq!( "World!", hello_prop );
    /// ```
    pub fn get_owned< K : AsRef< str >, T : From< Value > >( &self, key : K ) -> Option< T >
    {
      self.0.get( key.as_ref() ).map( | arg | arg.to_owned().into() )
    }
  }

  impl core::ops::Deref for Props
  {
    type Target = HashMap< String, Value > ;
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  // qqq : make 0-arguments, 1-argument, 2-arguments, 3 arguments versions
  // aaa : done. now it works with the following variants:
  // fn(), fn(args), fn(props), fn(args, props), fn(context), fn(context, args), fn(context, props), fn(context, args, props)
  type RoutineWithoutContextFn = dyn Fn( ( Args, Props ) ) -> Result< () >;
  type RoutineWithContextFn = dyn Fn( ( Args, Props ), Context ) -> Result< () >;

  ///
  /// Routine handle.
  ///
  /// ```
  /// # use wca::Routine;
  /// let routine = Routine::new
  /// (
  ///   |( args, props )|
  ///   {
  ///     // Do what you need to do
  ///
  ///     Ok( () )
  ///   }
  /// );
  /// ```
  ///
  /// ```
  /// # use wca::Routine;
  /// let routine = Routine::new_with_ctx
  /// (
  ///   | ( args, props ), ctx |
  ///   {
  ///     // Do what you need to do
  ///
  ///     Ok( () )
  ///   }
  /// );

  pub struct Handler< I, O >( Box< dyn Fn( I ) -> O > );

  impl< I, O > std::fmt::Debug for Handler< I, O >
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "Handler" ).finish_non_exhaustive()
    }
  }

  // without context
  impl< F, R > From< F > for Handler< (), R >
  where
    R : IntoResult + 'static,
    F : Fn() -> R + 'static,
  {
    fn from( value : F ) -> Self
    {
      Self( Box::new( move | () | value() ) )
    }
  }

  impl< F, R > From< F > for Handler< Args, R >
  where
    R : IntoResult + 'static,
    F : Fn( Args ) -> R + 'static,
  {
    fn from( value : F ) -> Self
    {
      Self( Box::new( value ) )
    }
  }

  impl< F, R > From< F > for Handler< Props, R >
    where
      R : IntoResult + 'static,
      F : Fn( Props ) -> R + 'static,
  {
    fn from( value : F ) -> Self
    {
      Self( Box::new( value ) )
    }
  }

  impl< F, R > From< F > for Handler< ( Args, Props ), R >
    where
      R : IntoResult + 'static,
      F : Fn( Args, Props ) -> R + 'static,
  {
    fn from( value : F ) -> Self
    {
      Self( Box::new( move |( a, p )| value( a, p ) ) )
    }
  }

  // with context
  impl< F, R > From< F > for Handler< Context, R >
  where
    R : IntoResult + 'static,
    F : Fn( Context ) -> R + 'static,
  {
    fn from( value : F ) -> Self
    {
      Self( Box::new( value ) )
    }
  }

  impl< F, R > From< F > for Handler< ( Context, Args ), R >
  where
    R : IntoResult + 'static,
    F : Fn( Context, Args ) -> R + 'static,
  {
    fn from( value : F ) -> Self
    {
      Self( Box::new( move |( ctx, a )| value( ctx, a ) ) )
    }
  }

  impl< F, R > From< F > for Handler< ( Context, Props ), R >
  where
    R : IntoResult + 'static,
    F : Fn( Context, Props ) -> R + 'static,
  {
    fn from( value : F ) -> Self
    {
      Self( Box::new( move |( ctx, a )| value( ctx, a ) ) )
    }
  }

  impl< F, R > From< F > for Handler< ( Context, Args, Props ), R >
  where
    R : IntoResult + 'static,
    F : Fn( Context, Args, Props ) -> R + 'static,
  {
    fn from( value : F ) -> Self
    {
      Self( Box::new( move |( c, a, p )| value( c, a, p ) ) )
    }
  }

  impl< I, O > From< Handler< I, O > > for Routine
  where
    I : 'static,
    O : IntoResult + 'static,
    Routine : From< Box< dyn Fn( I ) -> Result< () > > >,
  {
    fn from( value : Handler< I, O > ) -> Self
    {
      Routine::from( Box::new( move | x | value.0( x ).into_result() ) )
    }
  }

  /// Represents different types of routines.
  ///
  /// - `WithoutContext`: A routine that does not require any context.
  /// - `WithContext`: A routine that requires a context.
// qqq : for Bohdan : instead of array of Enums, lets better have 5 different arrays of different Routine and no enum
  // to use statical dispatch
  #[ derive( Clone ) ]
  pub enum Routine
  {
    /// Routine without context
    WithoutContext( Rc< RoutineWithoutContextFn > ),
    /// Routine with context
    WithContext( Rc< RoutineWithContextFn > ),
  }

  impl std::fmt::Debug for Routine
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      match self
      {
        Routine::WithoutContext( _ ) => f.debug_struct( "Routine::WithoutContext" ).finish_non_exhaustive(),
        Routine::WithContext( _ ) => f.debug_struct( "Routine::WithContext" ).finish_non_exhaustive(),
      }
    }
  }

  // without context
  impl From< Box< dyn Fn( () ) -> Result< () > > > for Routine
  {
    fn from( value : Box< dyn Fn( () ) -> Result< () > > ) -> Self
    {
      Self::WithoutContext( Rc::new( move | _ | { value( () )?; Ok( () ) } ) )
    }
  }

  impl From< Box< dyn Fn( Args ) -> Result< () > > > for Routine
  {
    fn from( value : Box< dyn Fn( Args ) -> Result< () > > ) -> Self
    {
      Self::WithoutContext( Rc::new( move |( a, _ )| { value( a )?; Ok( () ) } ) )
    }
  }

  impl From< Box< dyn Fn( Props ) -> Result< () > > > for Routine
  {
    fn from( value : Box< dyn Fn( Props ) -> Result< () > > ) -> Self
    {
      Self::WithoutContext( Rc::new( move |( _, p )| { value( p )?; Ok( () ) } ) )
    }
  }

  impl From< Box< dyn Fn(( Args, Props )) -> Result< () > > > for Routine
  {
    fn from( value : Box< dyn Fn(( Args, Props )) -> Result< () > > ) -> Self
    {
      Self::WithoutContext( Rc::new( move |( a, p )| { value(( a, p ))?; Ok( () ) } ) )
    }
  }

  // with context
  impl From< Box< dyn Fn( Context ) -> Result< () > > > for Routine
  {
    fn from( value : Box< dyn Fn( Context ) -> Result< () > > ) -> Self
    {
      Self::WithContext( Rc::new( move | _, ctx | { value( ctx )?; Ok( () ) } ) )
    }
  }

  impl From< Box< dyn Fn(( Context, Args )) -> Result< () > > > for Routine
  {
    fn from( value : Box< dyn Fn(( Context, Args )) -> Result< () > > ) -> Self
    {
      Self::WithContext( Rc::new( move | ( a, _ ), ctx | { value(( ctx, a ))?; Ok( () ) } ) )
    }
  }

  impl From< Box< dyn Fn(( Context, Props )) -> Result< () > > > for Routine
  {
    fn from( value : Box< dyn Fn(( Context, Props )) -> Result< () > > ) -> Self
    {
      Self::WithContext( Rc::new( move | ( _, p ), ctx | { value(( ctx, p ))?; Ok( () ) } ) )
    }
  }

  impl From< Box< dyn Fn(( Context, Args, Props )) -> Result< () > > > for Routine
  {
    fn from( value : Box< dyn Fn(( Context, Args, Props )) -> Result< () > > ) -> Self
    {
      Self::WithContext( Rc::new( move | ( a, p ), ctx | { value(( ctx, a, p ))?; Ok( () ) } ) )
    }
  }


  // qqq : why Rc is necessary? why not just box?
  // aaa : to be able to clone Routines

  impl Routine
  {
    ///
    /// Create new routine.
    ///
    /// ```
    /// # use wca::Routine;
    /// let routine = Routine::new
    /// (
    ///   |( args, props )|
    ///   {
    ///     // Do what you need to do
    ///
    ///     Ok( () )
    ///   }
    /// );
    /// ```

    pub fn new< F >( callback : F ) -> Self
    where
      F : Fn(( Args, Props )) -> Result< () > + 'static,
    {
      Routine::WithoutContext( Rc::new( callback ) )
    }

    ///
    /// Create new routine with context.
    ///
    /// ```
    /// # use wca::Routine;
    /// let routine = Routine::new_with_ctx
    /// (
    ///   | ( args, props ), ctx |
    ///   {
    ///     // Do what you need to do
    ///
    ///     Ok( () )
    ///   }
    /// );
    /// ```

    pub fn new_with_ctx< F >( callback : F ) -> Self
    where
      F : Fn( ( Args, Props ), Context ) -> Result< () > + 'static,
    {
      Routine::WithContext( Rc::new( callback ) )
    }
  }

  impl PartialEq for Routine
  {
    fn eq( &self, other : &Self ) -> bool
    {
      // We can't compare closures. Because every closure has a separate type, even if they're identical.
      // Therefore, we check that the two Rc's point to the same closure (allocation).
      #[ allow( clippy::vtable_address_comparisons ) ]
      match ( self, other )
      {
        ( Routine::WithContext( this ), Routine::WithContext( other ) ) => Rc::ptr_eq( this, other ),
        ( Routine::WithoutContext( this ), Routine::WithoutContext( other ) ) => Rc::ptr_eq( this, other ),
        _ => false
      }
    }
  }

  impl Eq for Routine {}

  trait IntoResult
  {
    fn into_result( self ) -> Result< () >;
  }

  impl IntoResult for std::convert::Infallible { fn into_result( self ) -> Result< () > { Ok( () ) } }
  impl IntoResult for () { fn into_result( self ) -> Result< () > { Ok( () ) } }
  impl< E : std::fmt::Display > IntoResult for Result< (), E > { fn into_result( self ) -> Result< () > { self.map_err( | e | anyhow!( "{e}" )) } }
}

//

crate::mod_interface!
{
  exposed use Routine;
  exposed use Handler;
  exposed use Args;
  exposed use Props;
}
