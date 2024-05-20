use super::*;
use macro_tools::{ attr, diag, generic_params, item_struct, struct_like::StructLike, Result };

// xxx2 : get complete From for enums

//

pub fn from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  use macro_tools::quote::ToTokens;

  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed
  {
    StructLike::Unit( ref item ) | StructLike::Struct( ref item ) =>
    {

      let mut field_types = item_struct::field_types( &item );
      let field_names = item_struct::field_names( &item );

      match ( field_types.len(), field_names )
      {
        ( 0, _ ) =>
        generate_unit
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
        ),
        ( 1, Some( mut field_names ) ) =>
        generate_from_single_field_named
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          field_names.next().unwrap(), // xxx : ?
          &field_types.next().unwrap(),
        ),
        ( 1, None ) =>
        generate_from_single_field
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          &field_types.next().unwrap(),
        ),
        ( _, Some( field_names ) ) =>
        generate_from_multiple_fields_named
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          field_names,
          field_types,
        ),
        ( _, None ) =>
        generate_from_multiple_fields
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          field_types,
        ),
      }

    },
    StructLike::Enum( ref item ) =>
    {

      let mut map = std::collections::HashMap::new();
      item.variants.iter().for_each( | variant |
      {
        map
        .entry( variant.fields.to_token_stream().to_string() )
        .and_modify( | e | *e += 1 )
        .or_insert( 1 );
      });

      let variants = item.variants.iter().map( | variant |
      {
        if map[ &variant.fields.to_token_stream().to_string() ] <= 1
        {
          variant_generate
          (
            item_name,
            &generics_impl,
            &generics_ty,
            &generics_where,
            variant,
          )
        }
        else
        {
          qt!{}
        }
      });
      qt!
      {
        #( #variants )*
      }
    },
  };

  if has_debug
  {
    let about = format!( "derive : From\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

// qqq  : document, add example of generated code
fn variant_generate
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variant : &syn::Variant,
)
-> proc_macro2::TokenStream
{
  let variant_name = &variant.ident;
  let fields = &variant.fields;

  if fields.len() <= 0
  {
    return qt!{}
  }

  let ( args, use_src ) = if fields.len() == 1
  {
    let field = fields.iter().next().unwrap();
    (
      qt!{ #field },
      qt!{ src },
    )
  }
  else
  {
    let src_i = ( 0..fields.len() ).map( | e |
    {
      let i = syn::Index::from( e );
      qt!{ src.#i, }
    });
    (
      qt!{ #fields },
      qt!{ #( #src_i )* },
      // qt!{ src.0, src.1 },
    )
  };

  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > From< #args > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline ]
      fn from( src : #args ) -> Self
      {
        Self::#variant_name( #use_src )
      }
    }
  }

}

// qqq  : document, add example of generated code -- done
/// Generates `From` implementation for tuple structs with a single field
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::From;
/// #[ derive( From ) ]
/// pub struct IsTransparent
/// {
///   value : bool,
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub struct IsTransparent
/// {
///   value : bool,
/// }
/// #[ automatically_derived ]
/// impl From< bool > for IsTransparent
/// {
///   #[ inline( always ) ]
///   fn from( src : bool ) -> Self
///   {
///     Self { value : src }
///   }
/// }
/// ```
///
fn generate_from_single_field_named
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_name : &syn::Ident,
  field_type : &syn::Type,
)
-> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > From< #field_type > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn from( src : #field_type ) -> Self
      {
        Self { #field_name : src }
      }
    }
  }
}

// qqq  : document, add example of generated code -- done
/// Generates `From`` implementation for structs with a single named field
///
/// # Example of generated code
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::From;
/// #[ derive( From ) ]
/// pub struct IsTransparent( bool );
/// ```
/// 
/// ## Output
/// ```rust
/// pub struct IsTransparent( bool );
/// #[ automatically_derived ]
/// impl From< bool > for IsTransparent
/// {
///   #[ inline( always ) ]
///   fn from( src : bool ) -> Self
///   {
///     Self( src )
///   }
/// }
/// ```
///
fn generate_from_single_field
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_type : &syn::Type,
)
-> proc_macro2::TokenStream
{

  qt!
  {
    #[automatically_derived]
    impl< #generics_impl > From< #field_type > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn from( src : #field_type ) -> Self
      {
        Self( src )
      }
    }
  }
}

// qqq : document, add example of generated code -- done
/// Generates `From` implementation for structs with multiple named fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::From;
/// #[ derive( From ) ]
/// pub struct Struct
/// {
///   value1 : bool,
///   value2 : i32,
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct
/// {
///   value1 : bool,
///   value2 : i32,
/// }
/// impl From< ( bool, i32 ) > for Struct
/// {
///   #[ inline( always ) ]
///   fn from( src : ( bool, i32 ) ) -> Self
///   {
///     Struct
///     {
///       value1 : src.0,
///       value2 : src.1,
///     }
///   }
/// }
/// ```
fn generate_from_multiple_fields_named< 'a >
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_names : Box< dyn macro_tools::IterTrait< 'a, &'a syn::Ident > + '_ >,
  field_types : impl macro_tools::IterTrait< 'a, &'a syn::Type >,
)
-> proc_macro2::TokenStream
{

  let params : Vec< proc_macro2::TokenStream > = field_names
  .enumerate()
  .map(| ( index, field_name ) |
  {
    let index = index.to_string().parse::< proc_macro2::TokenStream >().unwrap();
    qt! { #field_name : src.#index }
  })
  .collect();

  let field_types : Vec< _ > = field_types.collect();
  qt!
  {
    impl< #generics_impl > From< (# ( #field_types ),* ) > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn from( src : ( #( #field_types ),* ) ) -> Self
      {
        #item_name { #(#params),* }
      }
    }
  }

}

// qqq  : document, add example of generated code -- done
/// Generates `From` implementation for tuple structs with multiple fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::From;
/// #[ derive( From ) ]
/// pub struct Struct( bool, i32 );
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct( bool, i32 );
/// impl From< ( bool, i32 ) > for Struct
/// {
///   #[ inline( always ) ]
///   fn from( src : ( bool, i32 ) ) -> Self
///   {
///     Struct( src.0, src.1 )
///   }
/// }
/// ```
///
fn generate_from_multiple_fields< 'a >
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_types : impl macro_tools::IterTrait< 'a, &'a macro_tools::syn::Type >,
)
-> proc_macro2::TokenStream
{

  let params : Vec< proc_macro2::TokenStream > = ( 0..field_types.len() )
  .map( | index |
  {
    let index = index.to_string().parse::< proc_macro2::TokenStream >().unwrap();
    qt!( src.#index )
  })
  .collect();

  let field_types : Vec< _ > = field_types.collect();

  qt!
  {
    impl< #generics_impl > From< (# ( #field_types ),* ) > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn from( src : ( #( #field_types ),* ) ) -> Self
      {
        #item_name( #( #params ),* )
      }
    }
  }
}

// qqq  : document, add example of generated code -- done
/// Generates `From` implementation for unit structs
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::From;
/// #[ derive( From ) ]
/// pub struct IsTransparent;
/// ```
///
/// ## Output
/// ```rust
/// pub struct IsTransparent;
/// impl From< () > for IsTransparent
/// {
///   #[ inline( always ) ]
///   fn from( src : () ) -> Self
///   {
///     Self
///   }
/// }
/// ```
///
fn generate_unit
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
)
-> proc_macro2::TokenStream
{
  qt!
  {
    impl< #generics_impl > From< () > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn from( src : () ) -> Self
      {
        Self
      }
    }
  }
}
