use super::*;

use collection_tools::HashMap;

impl< K, V > Container for collection_tools::HashMap< K, V >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  type Entry = ( K, V );
  type Val = V;

  #[ inline( always ) ]
  fn entry_to_val( e : Self::Entry ) -> Self::Val
  {
    e.1
  }

}

impl< K, V > ContainerAdd for collection_tools::HashMap< K, V >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  // type Entry = ( K, V );
  // type Val = V;

  #[ inline( always ) ]
  fn add( &mut self, ( k, v ) : Self::Entry ) -> bool
  {
    self.insert( k, v ).map_or_else( || true, | _ | false )
  }

}

impl< K, V > ContainerAssign for collection_tools::HashMap< K, V >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  // type Entry = ( K, V );

  fn assign< Elements >( &mut self, elements : Elements ) -> usize
  where
    Elements : IntoIterator< Item = Self::Entry >
  {
    let initial_len = self.len();
    self.extend( elements );
    self.len() - initial_len
  }
}

/// A trait for types that behave like hash maps, supporting insertion and custom forming behaviors.
///
/// This trait allows for generic operations on hash map-like data structures, enabling the insertion
/// of key-value pairs and the creation of formers for more complex construction patterns.
///
/// # Type Parameters
/// - `K`: The type of keys stored in the hash map. Must implement `Eq` and `Hash`.
/// - `E`: The type of elements (values) stored in the hash map.
pub trait HashMapLike< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  Self : Sized + Default,
{

  /// Inserts a key-value pair into the map.
  fn insert( &mut self, k : K, e : E ) -> Option< E >;

}

impl< K, E > HashMapLike< K, E > for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  Self : Sized + Default,
{

  #[ inline( always ) ]
  fn insert( &mut self, k : K, e : E ) -> Option< E >
  {
    HashMap::insert( self, k, e )
  }

}

// = storage

impl< K, E > Storage
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  // type Types = HashMapDefinition< K, E >;
  // type Formed = HashMap< K, E >;
  type Preformed = HashMap< K, E >;
}

impl< K, E > StoragePreform
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  // type Preformed = HashMap< K, E >;
  // fn preform( self ) -> < < Self as Storage >::Definition as FormerDefinitionTypes >::Formed
  fn preform( self ) -> Self::Preformed
  {
    self
  }
}

// = definition

#[ derive( Debug, Default ) ]
pub struct HashMapDefinition< K, E, Context = (), Formed = HashMap< K, E >, End = ReturnStorage >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : FormingEnd< HashMapDefinitionTypes< K, E, Context, Formed > >,
{
  _phantom : core::marker::PhantomData< ( K, E, Context, Formed, End ) >,
}

impl< K, E, Context, Formed, End > FormerDefinition
for HashMapDefinition< K, E, Context, Formed, End >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : FormingEnd< HashMapDefinitionTypes< K, E, Context, Formed > >,
{

  type Storage = HashMap< K, E >;
  type Formed = Formed;
  type Context = Context;

  type Types = HashMapDefinitionTypes< K, E, Context, Formed >;
  type End = End;

}

// = definition types

#[ derive( Debug, Default ) ]
pub struct HashMapDefinitionTypes< K, E, Context = (), Formed = HashMap< K, E > >
{
  _phantom : core::marker::PhantomData< ( K, E, Context, Formed ) >,
}

impl< K, E, Context, Formed > FormerDefinitionTypes
for HashMapDefinitionTypes< K, E, Context, Formed >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = HashMap< K, E >;
  type Formed = Formed;
  type Context = Context;
}

// = mutator

impl< K, E, Context, Formed > FormerMutator
for HashMapDefinitionTypes< K, E, Context, Formed >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
}

// = Entity To

impl< K, E, Definition > EntityToFormer< Definition > for HashMap< K, E >
// where
//   K : ::core::cmp::Eq + ::core::hash::Hash,
//   Definition : FormerDefinition< Storage = HashMap< K, E >, Formed = () >,
//   < Definition as definition::FormerDefinition>::End : Fn( HashMap< K, E >, Option< Definition::Context > ),
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  Definition : FormerDefinition
  <
    Storage = HashMap< K, E >,
    Types = HashMapDefinitionTypes
    <
      K,
      E,
      < Definition as definition::FormerDefinition >::Context,
      < Definition as definition::FormerDefinition >::Formed,
    >,
  >,
  Definition::End : forming::FormingEnd< Definition::Types >,
{
  type Former = HashMapAsSubformer< K, E, Definition::Context, Definition::Formed, Definition::End >;
}

impl< K, E > crate::EntityToStorage
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Storage = HashMap< K, E >;
}

impl< K, E, Context, Formed, End > crate::EntityToDefinition< Context, Formed, End >
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
  End : crate::FormingEnd< HashMapDefinitionTypes< K, E, Context, Formed > >,
{
  type Definition = HashMapDefinition< K, E, Context, Formed, End >;
  type Types = HashMapDefinitionTypes< K, E, Context, Formed >;
}

impl< K, E, Context, Formed > crate::EntityToDefinitionTypes< Context, Formed >
for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  type Types = HashMapDefinitionTypes< K, E, Context, Formed >;
}

// = subformer

/// A builder for constructing hash map-like structures with a fluent interface.
///
/// `HashMapAsSubformer` leverages the `HashMapLike` trait to enable a flexible and customizable
/// way to build hash map-like structures. It supports the chaining of insert operations and
/// allows for the definition of custom end actions to finalize the building process.
///
/// # Type Parameters
/// - `K`: Key type, must implement `Eq` and `Hash`.
/// - `E`: Entry (value) type.
/// - `Formed`: The hash map-like formed being built.
/// - `Context`: Type of the optional context used during the building process.
/// - `End`: End-of-forming action to be executed upon completion.
///
/// # Examples
/// ```
/// # #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
/// # {
/// # use test_tools::exposed::*;
///
/// #[ derive( Debug, PartialEq, former::Former ) ]
/// pub struct StructWithMap
/// {
///   #[ container( definition = former::HashMapAsSubformer ) ]
///   map : std::collections::HashMap< &'static str, &'static str >,
/// }
///
/// let struct1 = StructWithMap::former()
/// .map()
///   .insert( "a", "b" )
///   .insert( "c", "d" )
///   .end()
/// .form()
/// ;
/// assert_eq!( struct1, StructWithMap { map : hmap!{ "a" => "b", "c" => "d" } } );
///
/// # }
/// ```

// pub type HashMapAsSubformer< K, E, Context, End > = ContainerSubformer::< ( K, E ), HashMapDefinition< K, E, Context, End > >;

// zzz : update documentation
// pub type HashMapAsSubformer< K, E, Context, End > = ContainerSubformer::< K, HashMapDefinition< K, E, Context, End > >;
pub type HashMapAsSubformer< K, E, Context, Formed, End > =
ContainerSubformer::< ( K, E ), HashMapDefinition< K, E, Context, Formed, End > >;

// = extension

pub trait HashMapExt< K, E > : sealed::Sealed
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn former() -> HashMapAsSubformer< K, E, (), HashMap< K, E >, ReturnStorage >;
}

impl< K, E > HashMapExt< K, E > for HashMap< K, E >
where
  K : ::core::cmp::Eq + ::core::hash::Hash,
{
  fn former() -> HashMapAsSubformer< K, E, (), HashMap< K, E >, ReturnStorage >
  {
    HashMapAsSubformer::< K, E, (), HashMap< K, E >, ReturnStorage >::new( ReturnStorage::default() )
  }
}

mod sealed
{
  use super::HashMap;
  pub trait Sealed {}
  impl< K, E > Sealed for HashMap< K, E > {}
}
