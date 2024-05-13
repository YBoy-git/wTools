#![ allow( dead_code ) ]
//! # Builder Pattern Implementation with Former
//!
//! This module demonstrates the use of the `Former` trait to apply the builder pattern for Rust structs.
//! The `Former` trait simplifies the instantiation of structs by enabling a fluent, method-chaining approach
//! to set fields before finalizing the instance with `.form()`. It is particularly useful for structs with optional fields
//! or when a clear and concise way to instantiate complex data structures is needed.
//!
//! ## How Former Works
//!
//! - **Trait Derivation** : By deriving `Former` on a struct, you automatically generate builder methods for each field.
//! - **Fluent Interface** : Each field's builder method allows for setting the value of that field and returns a mutable reference to the builder,
//!   enabling method chaining.
//! - **Optional Fields** : Optional fields can be easily handled without needing to explicitly set them to `None`.
//! - **Finalization** : The `.form()` method finalizes the building process and returns the constructed struct instance.
//!
//! This approach abstracts away the need for manually implementing a builder for each struct, making code more readable and maintainable.
//!

#[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
fn main() {}
#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{

  // Use attribute debug to print expanded code.
  #[ derive( Debug, PartialEq ) ]
  pub struct UserProfile
  {
    age : i32,
    username : String,
    bio_optional : Option< String >, // Fields could be optional
  }

  impl UserProfile
  where
  {
    #[ inline( always ) ]
    pub fn former() -> UserProfileFormer<
      UserProfileFormerDefinition< (), UserProfile, former::ReturnPreformed >
    >
    {
      UserProfileFormer::< UserProfileFormerDefinition< (), UserProfile, former::ReturnPreformed > >::
      new_coercing(former::ReturnPreformed)
    }
  }

  // = entity to

  impl< Definition > former::EntityToFormer< Definition > for UserProfile
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage >,
  {
    type Former = UserProfileFormer< Definition >;
  }

  impl former::EntityToStorage for UserProfile
  where
  {
    type Storage = UserProfileFormerStorage;
  }

  impl< Context, Formed, End > former::EntityToDefinition< Context, Formed, End > for UserProfile< >
  where
    End : former::FormingEnd< UserProfileFormerDefinitionTypes< Context, Formed > >,
  {
    type Definition = UserProfileFormerDefinition< Context, Formed, End >;
    type Types = UserProfileFormerDefinitionTypes< Context, Formed >;
  }

  // = definition

  #[derive(Debug)]
  pub struct UserProfileFormerDefinitionTypes< Context = (), Formed = UserProfile, >
  where
  {
    _phantom : core::marker::PhantomData< (*const Context, *const Formed) >,
  }

  impl< Context, Formed, > ::core::default::Default for UserProfileFormerDefinitionTypes< Context, Formed, >
  where
  {
    fn default() -> Self
    {
      Self
      {
        _phantom : core::marker::PhantomData,
      }
    }
  }

  impl< Context, Formed, > former::FormerDefinitionTypes for UserProfileFormerDefinitionTypes< Context, Formed, >
  where
  {
    type Storage = UserProfileFormerStorage;
    type Formed = Formed;
    type Context = Context;
  }

  #[derive(Debug)]
  pub struct UserProfileFormerDefinition< Context = (), Formed = UserProfile, End = former::ReturnPreformed, >
  where
  {
    _phantom : core::marker::PhantomData< (*const Context, *const Formed, *const End) >,
  }

  impl< Context, Formed, End, > ::core::default::Default for UserProfileFormerDefinition< Context, Formed, End, >
  where
  {
    fn default() -> Self
    {
      Self
      {
        _phantom : core::marker::PhantomData,
      }
    }
  }

  impl< Context, Formed, End, > former::FormerDefinition for UserProfileFormerDefinition< Context, Formed, End, >
  where
    End : former::FormingEnd< UserProfileFormerDefinitionTypes< Context, Formed, > >,
  {
    type Types = UserProfileFormerDefinitionTypes< Context, Formed, >;
    type End = End;
    type Storage = UserProfileFormerStorage;
    type Formed = Formed;
    type Context = Context;
  }

  impl< Context, Formed, > former::FormerMutator for UserProfileFormerDefinitionTypes< Context, Formed, >
  where
  {}

  // = storage

  pub struct UserProfileFormerStorage
  where
  {
    pub age : ::core::option::Option< i32 >,
    pub username : ::core::option::Option< String >,
    pub bio_optional : Option< String >,
  }

  impl ::core::default::Default for UserProfileFormerStorage
  where
  {
    #[ inline( always ) ]
    fn default() -> Self
    {
      Self
      {
        age : ::core::option::Option::None,
        username : ::core::option::Option::None,
        bio_optional : ::core::option::Option::None,
      }
    }
  }

  impl former::Storage for UserProfileFormerStorage
  where
  {
    type Preformed = UserProfile;
  }

  impl former::StoragePreform for UserProfileFormerStorage
  where
  {
    // type Preformed = UserProfile;
    fn preform(mut self) -> Self::Preformed
    {
      let age = if self.age.is_some()
      {
        self.age.take().unwrap()
      }
      else
      {
        {
          trait MaybeDefault< T >
          {
            fn maybe_default(self : &Self) -> T
            {
              panic!("Field 'age' isn't initialized")
            }
          }
          impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T >
          {}
          impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
          where T : ::core::default::Default,
          {
            fn maybe_default(self : &Self) -> T
            {
              T::default()
            }
          }
          (&::core::marker::PhantomData::< i32 >).maybe_default()
        }
      };
      let username = if self.username.is_some()
      {
        self.username.take().unwrap()
      }
      else
      {
        {
          trait MaybeDefault< T >
          {
            fn maybe_default(self : &Self) -> T
            {
              panic!("Field 'username' isn't initialized")
            }
          }
          impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T >
          {}
          impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
          where T : ::core::default::Default,
          {
            fn maybe_default(self : &Self) -> T
            {
              T::default()
            }
          }
          (&::core::marker::PhantomData::< String >).maybe_default()
        }
      };
      let bio_optional = if self.bio_optional.is_some()
      {
        ::core::option::Option::Some(self.bio_optional.take().unwrap())
      }
      else
      {
        ::core::option::Option::None
      };
      let result = UserProfile::<>
      {
        age,
        username,
        bio_optional,
      };
      return result;
    }
  }

  pub struct UserProfileFormer< Definition = UserProfileFormerDefinition< (), UserProfile, former::ReturnPreformed >, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage >,
  {
    pub storage : Definition::Storage,
    pub context : core::option::Option< Definition::Context >,
    pub on_end : core::option::Option< Definition::End >,
  }

  impl< Definition, > UserProfileFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage >, Definition::Types : former::FormerDefinitionTypes< Storage = UserProfileFormerStorage >,
  {
    #[ inline( always ) ]
    pub fn new(on_end : Definition::End) -> Self
    {
      Self::begin_coercing(None, None, on_end)
    }

    #[ inline( always ) ]
    pub fn new_coercing< IntoEnd >(end : IntoEnd) -> Self
    where IntoEnd : Into< Definition::End >,
    {
      Self::begin_coercing(None, None, end,)
    }

    #[ inline( always ) ]
    pub fn begin(mut storage : core::option::Option< Definition::Storage >, context : core::option::Option< Definition::Context >, on_end : <Definition as former::FormerDefinition>::End,) -> Self
    {
      if storage.is_none()
      {
        storage = Some(::core::default::Default::default());
      }
      Self
      {
        storage : storage.unwrap(),
        context : context,
        on_end : ::core::option::Option::Some(on_end),
      }
    }

    #[ inline( always ) ]
    pub fn begin_coercing< IntoEnd >(mut storage : core::option::Option< Definition::Storage >, context : core::option::Option< Definition::Context >, on_end : IntoEnd,) -> Self
    where IntoEnd : ::core::convert::Into< <Definition as former::FormerDefinition>::End >,
    {
      if storage.is_none()
      {
        storage = Some(::core::default::Default::default());
      }
      Self
      {
        storage : storage.unwrap(),
        context : context,
        on_end : ::core::option::Option::Some(::core::convert::Into::into(on_end)),
      }
    }

    #[ inline( always ) ]
    pub fn form(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
    {
      self.end()
    }

    #[ inline( always ) ]
    pub fn end(mut self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
    {
      let on_end = self.on_end.take().unwrap();
      let mut context = self.context.take();
      <Definition::Types as former::FormerMutator>::form_mutation(&mut self.storage, &mut context);
      former::FormingEnd::<Definition::Types>::call(&on_end, self.storage, context)
    }

    #[ inline( always ) ]
    pub fn age< Src >(mut self, src : Src) -> Self
    where Src : ::core::convert::Into< i32 >,
    {
      debug_assert!(self.storage.age.is_none());
      self.storage.age = ::core::option::Option::Some(::core::convert::Into::into(src));
      self
    }

    #[ inline( always ) ]
    pub fn username< Src >(mut self, src : Src) -> Self
    where Src : ::core::convert::Into< String >,
    {
      debug_assert!(self.storage.username.is_none());
      self.storage.username = ::core::option::Option::Some(::core::convert::Into::into(src));
      self
    }

    #[ inline( always ) ]
    pub fn bio_optional< Src >(mut self, src : Src) -> Self
    where Src : ::core::convert::Into< String >,
    {
      debug_assert!(self.storage.bio_optional.is_none());
      self.storage.bio_optional = ::core::option::Option::Some(::core::convert::Into::into(src));
      self
    }
  }

  impl< Definition, > UserProfileFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage, Formed = UserProfile >,
  {
    pub fn preform(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
    {
      former::StoragePreform::preform(self.storage)
    }
  }

  impl< Definition, > UserProfileFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage, Formed = UserProfile, >,
  {
    #[ inline( always ) ]
    pub fn perform(self) -> Definition::Formed
    {
      let result = self.form();
      return result;
    }
  }

  impl< Definition > former::FormerBegin< Definition > for UserProfileFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage >,
  {
    #[ inline( always ) ]
    fn former_begin(storage : core::option::Option< Definition::Storage >, context : core::option::Option< Definition::Context >, on_end : Definition::End,) -> Self
    {
      debug_assert!(storage.is_none());
      Self::begin(None, context, on_end)
    }
  }

  // = as subformer

  pub type UserProfileAsSubformer< Superformer, End > =
  UserProfileFormer< UserProfileFormerDefinition< Superformer, Superformer, End, >, >;

  pub trait UserProfileAsSubformerEnd< SuperFormer >
  where
    Self : former::FormingEnd< UserProfileFormerDefinitionTypes< SuperFormer, SuperFormer >, >, {}

  impl< SuperFormer, T > UserProfileAsSubformerEnd< SuperFormer > for T
  where
    Self : former::FormingEnd< UserProfileFormerDefinitionTypes< SuperFormer, SuperFormer >, >,
  {}

  // = end

  let profile = UserProfile::former()
  .age( 30 )
  .username( "JohnDoe".to_string() )
  .bio_optional( "Software Developer".to_string() ) // Optionally provide a bio
  .form();
  dbg!( &profile );

  // Expected output:
  //
  // &profile = UserProfile {
  //   age: 30,
  //   username: "JohnDoe",
  //   bio_optional: Some("Software Developer"),
  // }

}
