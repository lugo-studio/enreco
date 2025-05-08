extern crate proc_macro;

use std::{ffi::CString, str::FromStr};

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(ToStringName)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
  let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

  let variants = if let syn::Data::Enum(e) = data {
    e.variants.into_iter().collect::<Vec<_>>()
  } else {
    panic!("Can only derive ToStringName on enums.");
  };

  let c_str_variants = variants.iter().map(|v| {
    let variant_ident = &v.ident;
    let string = variant_ident.to_string().to_case(Case::Title);
    let c_string = CString::from_str(&string).unwrap();
    let name = Literal::c_string(&c_string);
    quote! {
      #ident::#variant_ident => #name,
    }
  });

  let expanded = quote! {
    impl #ident {
      pub const fn cstr(self) -> &'static std::ffi::CStr {
        match self {
          #( #c_str_variants )*
        }
      }
    }

    impl godot::meta::AsArg<godot::prelude::StringName> for #ident {
      fn into_arg<'r>(self) -> <godot::prelude::StringName as godot::meta::ParamType>::Arg<'r>
      where
        Self: 'r,
      {
        self.cstr().into_arg()
      }
    }
  };

  // Hand the output tokens back to the compiler
  expanded.into()
}
