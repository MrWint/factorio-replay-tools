extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(ReadWriteEnumU8)]
pub fn derive_enum_u8(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

  let expanded = quote! {
    impl factorio_serialize::ReadWrite for #name {
      fn read<R: std::io::BufRead + std::io::Seek>(r: &mut factorio_serialize::Reader<R>) -> factorio_serialize::Result<Self> {
        let value = r.read_u8()?;
        Self::from_u8(value).ok_or_else(|| r.error_at(format!("value {:#x} is not a valid {}", value, stringify!(#name)), 1))
      }
      fn write<W: std::io::Write + std::io::Seek>(&self, w: &mut factorio_serialize::Writer<W>) -> factorio_serialize::Result<()> {
        w.write_u8(self.to_u8().unwrap())
      }
    }
  };

  TokenStream::from(expanded)
}

#[proc_macro_derive(ReadWriteEnumU16)]
pub fn derive_enum_u16(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

  let expanded = quote! {
    impl factorio_serialize::ReadWrite for #name {
      fn read<R: std::io::BufRead + std::io::Seek>(r: &mut factorio_serialize::Reader<R>) -> factorio_serialize::Result<Self> {
        let value = r.read_u16()?;
        Self::from_u16(value).ok_or_else(|| r.error_at(format!("value {:#x} is not a valid {}", value, stringify!(#name)), 1))
      }
      fn write<W: std::io::Write + std::io::Seek>(&self, w: &mut factorio_serialize::Writer<W>) -> factorio_serialize::Result<()> {
        w.write_u16(self.to_u16().unwrap())
      }
    }
  };

  TokenStream::from(expanded)
}

#[proc_macro_derive(ReadWriteStruct, attributes(negated_bool, space_optimized))]
pub fn derive_readwrite(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  match input.data {
    syn::Data::Struct(data) => {
      let punctuated =  match data.fields {
        syn::Fields::Named(fields) => fields.named,
        syn::Fields::Unnamed(_) => panic!("Can't use ReadWriteStruct on unnamed type {}.", stringify!(#name)),
        syn::Fields::Unit => panic!("Can't use ReadWriteStruct on unit type {}.", stringify!(#name)),
      };

      let read_tokens: proc_macro2::TokenStream = punctuated.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        if field.attrs.iter().any(|attr| attr.path.is_ident("negated_bool")) {
          match ty {
            syn::Type::Path(path_type) if path_type.path.is_ident("bool") => quote! { let #name = !r.read_bool()?; },
            _ => panic!("negated_bool is only allowed on bool type")
          }
        } else if field.attrs.iter().any(|attr| attr.path.is_ident("space_optimized")) {
          match ty {
            syn::Type::Path(path_type) if path_type.path.is_ident("u16") => quote! { let #name = r.read_opt_u16()?; },
            syn::Type::Path(path_type) if path_type.path.is_ident("u32") => quote! { let #name = r.read_opt_u32()?; },
            _ => panic!("space_optimized is only allowed on u16 and u32 types")
          }
        } else {
          quote! { let #name = <#ty>::read(r)?; }
        }
      }).collect();
      let write_tokens: proc_macro2::TokenStream = punctuated.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        if field.attrs.iter().any(|attr| attr.path.is_ident("negated_bool")) {
          match &field.ty {
            syn::Type::Path(path_type) if path_type.path.is_ident("bool") => quote! { w.write_bool(!self.#name)?; },
            _ => panic!("negated_bool is only allowed on bool type")
          }
        } else if field.attrs.iter().any(|attr| attr.path.is_ident("space_optimized")) {
          match &field.ty {
            syn::Type::Path(path_type) if path_type.path.is_ident("u16") => quote! { w.write_opt_u16(!self.#name)?; },
            syn::Type::Path(path_type) if path_type.path.is_ident("u32") => quote! { w.write_opt_u32(!self.#name)?; },
            _ => panic!("space_optimized is only allowed on u16 and u32 types")
          }
        } else {
          quote! { self.#name.write(w)?; }
        }
      }).collect();
      let struct_param_tokens: proc_macro2::TokenStream = punctuated.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        quote! { #name, }
      }).collect();

      let name = &input.ident;

      let expanded = quote! {
        impl factorio_serialize::ReadWrite for #name {
          fn read<R: std::io::BufRead + std::io::Seek>(r: &mut factorio_serialize::Reader<R>) -> factorio_serialize::Result<Self> {
            #read_tokens
            Ok(#name { #struct_param_tokens })
          }
          fn write<W: std::io::Write + std::io::Seek>(&self, w: &mut factorio_serialize::Writer<W>) -> factorio_serialize::Result<()> {
            #write_tokens
            Ok(())
          }
        }
      };

      TokenStream::from(expanded)
    },
    syn::Data::Enum(_) => panic!("Can't use ReadWriteStruct on enum type {}.", stringify!(#name)),
    syn::Data::Union(_) => panic!("Can't use ReadWriteStruct on union type {}.", stringify!(#name)),
  }
}
