use super::*;
use macro_tools::{ attr, diag, item_struct, struct_like, struct_like::StructLike, Result };

// xxx2 : get complete From for enums

//

pub fn from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let original_input = input.clone();

  // let parsed = syn::parse::< syn::ItemStruct >( input )?;
  // let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  // let item_name = &parsed.ident;
  // let mut field_types = item_struct::field_types( &parsed );
  // let field_names = item_struct::field_names( &parsed );

  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  // let mut field_types = parsed.field_types();
  // let field_names = parsed.field_names();

  let result = match parsed
  {
    StructLike::Unit( ref item ) | StructLike::Struct( ref item ) =>
    {

      let mut field_types = item_struct::field_types( &item );
      let field_names = item_struct::field_names( &item );

      match ( field_types.len(), field_names )
      {
        ( 0, _ ) =>
        generate_unit( item_name ),
        ( 1, Some( mut field_names ) ) =>
        generate_from_single_field_named( &field_types.next().unwrap(), field_names.next().unwrap(), item_name ),
        ( 1, None ) =>
        generate_from_single_field( &field_types.next().unwrap(), item_name ),
        ( _, Some( field_names ) ) =>
        generate_from_multiple_fields_named( field_types, field_names, item_name ),
        ( _, None ) =>
        generate_from_multiple_fields( field_types, item_name ),
      }

    },
    StructLike::Enum( ref item ) =>
    {
      let variants = item.variants.iter().map( | v | variant_generate( item_name, v ) );
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
  variant : &syn::Variant,
)
-> proc_macro2::TokenStream
{
  let variant_name = &variant.ident;
  let fields = &variant.fields;
  qt!
  {
    #[ automatically_derived ]
    impl From< #fields > for #item_name
    {
      #[ inline ]
      fn from( src : #fields ) -> Self
      {
        Self::#variant_name( src )
      }
    }
  }
}

// qqq  : document, add example of generated code
fn generate_from_single_field_named
(
  field_type : &syn::Type,
  field_name : &syn::Ident,
  item_name : &syn::Ident,
)
-> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    // impl From < i32 > for MyStruct
    impl From< #field_type > for #item_name
    {
      #[ inline( always ) ]
      // fn from( src: i32 ) -> Self
      fn from( src: #field_type ) -> Self
      {
        // Self { a: src }
        Self { #field_name: src }
      }
    }
  }
}

// qqq  : document, add example of generated code
fn generate_from_single_field
(
  field_type : &syn::Type,
  item_name : &syn::Ident,
) -> proc_macro2::TokenStream
{

  qt!
  {
    #[automatically_derived]
    // impl From< bool > for IsTransparent
    impl From< #field_type > for #item_name
    {
      #[ inline( always ) ]
      // fn from( src: bool ) -> Self
      fn from( src: #field_type ) -> Self
      {
        // Self(src)
        Self(src)
      }
    }
  }
}

// qqq : for Petro : document, add example of generated code
fn generate_from_multiple_fields_named< 'a >
(
  field_types : impl macro_tools::IterTrait< 'a, &'a syn::Type >,
  field_names : Box< dyn macro_tools::IterTrait< 'a, &'a syn::Ident > + '_ >,
  item_name : &syn::Ident
) -> proc_macro2::TokenStream
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
    // impl From< (i32, bool) > for StructNamedFields
    impl From< ( #( #field_types ),* ) > for #item_name
    {
      #[ inline( always ) ]
      // fn from( src: (i32, bool) ) -> Self
      fn from( src : ( #( #field_types ),* ) ) -> Self
      {
        // StructNamedFields{ a: src.0, b: src.1 }
        #item_name { #(#params),* }
      }
    }
  }

}

// qqq  : document, add example of generated code
fn generate_from_multiple_fields< 'a >
(
  field_types : impl macro_tools::IterTrait< 'a, &'a macro_tools::syn::Type >,
  item_name : &syn::Ident,
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
    // impl From< (i32, bool) > for StructWithManyFields
    impl From< (# ( #field_types ),* ) > for #item_name
    {
      #[ inline( always ) ]
      // fn from( src: (i32, bool) ) -> Self
      fn from( src : ( #( #field_types ),* ) ) -> Self
      {
        // StructWithManyFields( src.0, src.1 )
        #item_name( #( #params ),* )
      }
    }
  }
}

// qqq  : document, add example of generated code
fn generate_unit( item_name : &syn::Ident ) -> proc_macro2::TokenStream
{
  qt!
  {
    // impl From< () > for UnitStruct
    impl From< () > for #item_name
    {
      #[ inline( always ) ]
      fn from( src: () ) -> Self
      {
        Self
      }
    }
  }
}