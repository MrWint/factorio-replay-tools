extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, Expr};


#[proc_macro_derive(MapReadWriteStruct, attributes(assert_eq, vec_u16, vec_u32, space_optimized))]
pub fn derive_mapreadwrite(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  match input.data {
    syn::Data::Struct(data) => {
      let punctuated =  match data.fields {
        syn::Fields::Named(fields) => fields.named,
        syn::Fields::Unnamed(_) => panic!("Can't use MapReadWriteStruct on unnamed type {}.", input.ident),
        syn::Fields::Unit => panic!("Can't use MapReadWriteStruct on unit type {}.", input.ident),
      };

      let map_read_tokens: proc_macro2::TokenStream = punctuated.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let mut assert_eq_val = None;
        let mut read_tokens = quote! { let #name = <#ty>::map_read(input)?; };
        for attribute in &field.attrs {
          if attribute.path().is_ident("assert_eq") {
            assert_eq_val = Some(attribute.parse_args::<Expr>().unwrap());
          } else if attribute.path().is_ident("space_optimized") {
            read_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.is_ident("u16") => quote! { let #name = input.stream.read_opt_u16()?; },
              syn::Type::Path(path_type) if path_type.path.is_ident("u32") => quote! { let #name = input.stream.read_opt_u32()?; },
              syn::Type::Path(path_type) if path_type.path.is_ident("u64") => quote! { let #name = input.stream.read_opt_u64()?; },
              _ => panic!("space_optimized used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("vec_u16") {
            read_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { let #name = crate::map::map_read_vec_u16(input)?; },
              _ => panic!("vec_u16 used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("vec_u32") {
            read_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { let #name = crate::map::map_read_vec_u32(input)?; },
              _ => panic!("vec_u32 used on invalid type {:?}", ty),
            }
          }
        }

        if let Some(assert_eq_val) = assert_eq_val {
          let ident_name = name.to_string();
          read_tokens = quote! {
            #read_tokens
            assert_eq!(#name, #assert_eq_val, "{} is not {}", #ident_name, #assert_eq_val);
          }
        }

        read_tokens
      }).collect();
      let map_write_tokens: proc_macro2::TokenStream = punctuated.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let mut assert_eq_val = None;
        let mut write_tokens = quote! { self.#name.map_write(w)?; };
        for attribute in &field.attrs {
          if attribute.path().is_ident("assert_eq") {
            assert_eq_val = Some(attribute.parse_args::<Expr>().unwrap());
          } else if attribute.path().is_ident("space_optimized") {
            write_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.is_ident("u16") => quote! { w.stream.write_opt_u16(self.#name)?; },
              syn::Type::Path(path_type) if path_type.path.is_ident("u32") => quote! { w.stream.write_opt_u32(self.#name)?; },
              syn::Type::Path(path_type) if path_type.path.is_ident("u64") => quote! { w.stream.write_opt_u64(self.#name)?; },
              _ => panic!("space_optimized used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("vec_u16") {
            write_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { crate::map::map_write_vec_u16(&self.#name, w)?; },
              _ => panic!("vec_u16 used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("vec_u32") {
            write_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { crate::map::map_write_vec_u32(&self.#name, w)?; },
              _ => panic!("vec_u32 used on invalid type {:?}", ty),
            }
          }
        }

        if let Some(assert_eq_val) = assert_eq_val {
          let ident_name = name.to_string();
          write_tokens = quote! {
            assert_eq!(self.#name, #assert_eq_val, "{} is not {}", #ident_name, #assert_eq_val);
            #write_tokens
          }
        }

        write_tokens
      }).collect();
      let struct_param_tokens: proc_macro2::TokenStream = punctuated.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        quote! { #name, }
      }).collect();

      let name = &input.ident;

      let expanded = quote! {
        impl #impl_generics crate::map::MapReadWrite for #name #ty_generics #where_clause {
          fn map_read<R: std::io::BufRead + std::io::Seek>(input: &mut crate::map::MapDeserialiser<R>) -> crate::Result<Self> {
            #map_read_tokens
            Ok(#name { #struct_param_tokens })
          }
          fn map_write(&self, w: &mut crate::map::MapSerialiser) -> crate::Result<()> {
            #map_write_tokens
            Ok(())
          }
        }
      };

      TokenStream::from(expanded)
    },
    syn::Data::Enum(_) => panic!("Can't use MapReadWriteStruct on enum type {}.", input.ident),
    syn::Data::Union(_) => panic!("Can't use MapReadWriteStruct on union type {}.", input.ident),
  }
}

#[proc_macro_derive(MapReadWriteEnumU8)]
pub fn map_derive_enum_u8(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

  let expanded = quote! {
    impl crate::map::MapReadWrite for #name {
      fn map_read<R: std::io::BufRead + std::io::Seek>(r: &mut crate::map::MapDeserialiser<R>) -> crate::Result<Self> {
        let value = r.stream.read_u8()?;
        Self::from_u8(value).ok_or_else(|| r.stream.error_at(format!("value {:#x} is not a valid {}", value, stringify!(#name)), 1))
      }
      fn map_write(&self, w: &mut crate::map::MapSerialiser) -> crate::Result<()> {
        w.stream.write_u8(self.to_u8().unwrap())
      }
    }
  };

  TokenStream::from(expanded)
}
























#[proc_macro_derive(ReplayReadWriteEnumU8)]
pub fn replay_derive_enum_u8(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

  let expanded = quote! {
    impl crate::replay::ReplayReadWrite for #name {
      fn replay_read<R: std::io::BufRead + std::io::Seek>(r: &mut crate::replay::ReplayDeserialiser<R>) -> crate::Result<Self> {
        let value = r.stream.read_u8()?;
        Self::from_u8(value).ok_or_else(|| r.stream.error_at(format!("value {:#x} is not a valid {}", value, stringify!(#name)), 1))
      }
      fn replay_write(&self, w: &mut crate::replay::ReplaySerialiser) -> crate::Result<()> {
        w.stream.write_u8(self.to_u8().unwrap())
      }
    }
  };

  TokenStream::from(expanded)
}
#[proc_macro_derive(ReplayReadWriteEnumU16)]
pub fn replay_derive_enum_u16(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

  let expanded = quote! {
    impl crate::replay::ReplayReadWrite for #name {
      fn replay_read<R: std::io::BufRead + std::io::Seek>(r: &mut crate::replay::ReplayDeserialiser<R>) -> crate::Result<Self> {
        let value = r.stream.read_u16()?;
        Self::from_u16(value).ok_or_else(|| r.stream.error_at(format!("value {:#x} is not a valid {}", value, stringify!(#name)), 1))
      }
      fn replay_write(&self, w: &mut crate::replay::ReplaySerialiser) -> crate::Result<()> {
        w.stream.write_u16(self.to_u16().unwrap())
      }
    }
  };

  TokenStream::from(expanded)
}
#[proc_macro_derive(ReplayReadWriteEnumU32)]
pub fn replay_derive_enum_u32(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

  let expanded = quote! {
    impl crate::replay::ReplayReadWrite for #name {
      fn replay_read<R: std::io::BufRead + std::io::Seek>(r: &mut crate::replay::ReplayDeserialiser<R>) -> crate::Result<Self> {
        let value = r.stream.read_u32()?;
        Self::from_u32(value).ok_or_else(|| r.stream.error_at(format!("value {:#x} is not a valid {}", value, stringify!(#name)), 1))
      }
      fn replay_write(&self, w: &mut crate::replay::ReplaySerialiser) -> crate::Result<()> {
        w.stream.write_u32(self.to_u32().unwrap())
      }
    }
  };

  TokenStream::from(expanded)
}
#[proc_macro_derive(ReplayReadWriteTaggedUnion, attributes(tag_type))]
pub fn replay_derive_readwrite_union(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let enum_ident = &input.ident;

  let tag_type: syn::Type = input.attrs.iter().find(|attr| attr.path().is_ident("tag_type")).expect("No tag_type attribute found on ReplayReadWriteTaggedUnion").parse_args().expect("Unable to parse tag_type as identifier");

  match &input.data {
    syn::Data::Enum(data) => {
      let read_tokens: proc_macro2::TokenStream = data.variants.iter().map(|variant| {
        let name = &variant.ident;
        match &variant.fields {
          syn::Fields::Unit => quote! { #tag_type::#name => Ok(#enum_ident::#name), },
          syn::Fields::Unnamed(f) => {
            assert!(f.unnamed.len() == 1, "enum variant {} must contain exactly one unnamed field", input.ident);
            let field_type = &f.unnamed.first().unwrap().ty;
            quote! { #tag_type::#name => Ok(#enum_ident::#name(<#field_type>::replay_read(r)?)), }
          },
          syn::Fields::Named(_) => panic!("Can't use ReplayReadWriteTaggedUnion on named enum variants."),
        }
      }).collect();
      let write_tokens: proc_macro2::TokenStream = data.variants.iter().map(|variant| {
        let name = &variant.ident;
        match &variant.fields {
          syn::Fields::Unit => quote! { #enum_ident::#name => Ok(()), },
          syn::Fields::Unnamed(_) => quote! { #enum_ident::#name(enum_data) => enum_data.replay_write(w), },
          syn::Fields::Named(_) => panic!("Can't use ReplayReadWriteTaggedUnion on named enum variants."),
        }
      }).collect();
      let to_tag_tokens: proc_macro2::TokenStream = data.variants.iter().map(|variant| {
        let name = &variant.ident;
        quote! { #enum_ident::#name { .. } => <#tag_type>::#name, }
      }).collect();

      let expanded = quote! {
        impl #enum_ident {
          fn replay_read<R: std::io::BufRead + std::io::Seek>(action_type: #tag_type, action_type_pos: u64, r: &mut crate::replay::ReplayDeserialiser<R>) -> crate::Result<Self> {
            match action_type {
              #read_tokens
              _ => Err(crate::Error::custom(format!("Unsupported action type {:?}", action_type), action_type_pos)),
            }
          }
          fn replay_write(&self, w: &mut crate::replay::ReplaySerialiser) -> crate::Result<()> {
            match self {
              #write_tokens
            }
          }
          pub fn to_tag(&self) -> #tag_type {
            match self {
              #to_tag_tokens
            }
          }
        }
      };

      TokenStream::from(expanded)
    },
    _ => panic!("ReplayReadWriteTaggedUnion on {} can only be used on enums.", stringify!(#name)),
  }
}

#[proc_macro_derive(ReplayReadWriteStruct, attributes(assert_eq, compacted_sorted, vec_u8, vec_u16, vec_opt_u16, vec_u32, space_optimized, conditional_or_default))]
pub fn derive_replay_readwrite(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  match input.data {
    syn::Data::Struct(data) => {
      let punctuated =  match data.fields {
        syn::Fields::Named(fields) => fields.named,
        syn::Fields::Unnamed(_) => panic!("Can't use ReplayReadWriteStruct on unnamed type {}.", input.ident),
        syn::Fields::Unit => panic!("Can't use ReplayReadWriteStruct on unit type {}.", input.ident),
      };

      let replay_read_tokens: proc_macro2::TokenStream = punctuated.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let mut assert_eq_val = None;
        let mut conditional_val = None;
        let mut read_tokens = quote! { let #name = <#ty>::replay_read(input)?; };
        for attribute in &field.attrs {
          if attribute.path().is_ident("assert_eq") {
            assert_eq_val = Some(attribute.parse_args::<Expr>().unwrap());
          } else if attribute.path().is_ident("conditional_or_default") {
            conditional_val = Some(attribute.parse_args::<Expr>().unwrap());
          } else if attribute.path().is_ident("space_optimized") {
            read_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.is_ident("u16") => quote! { let #name = input.stream.read_opt_u16()?; },
              syn::Type::Path(path_type) if path_type.path.is_ident("u32") => quote! { let #name = input.stream.read_opt_u32()?; },
              syn::Type::Path(path_type) if path_type.path.is_ident("u64") => quote! { let #name = input.stream.read_opt_u64()?; },
              _ => panic!("space_optimized used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("vec_u8") {
            read_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { let #name = crate::replay::replay_read_vec_u8(input)?; },
              _ => panic!("vec_u8 used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("vec_u16") {
            read_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { let #name = crate::replay::replay_read_vec_u16(input)?; },
              _ => panic!("vec_u16 used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("vec_opt_u16") {
            read_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { let #name = crate::replay::replay_read_vec_opt_u16(input)?; },
              _ => panic!("vec_u16 used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("vec_u32") {
            read_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { let #name = crate::replay::replay_read_vec_u32(input)?; },
              _ => panic!("vec_u32 used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("compacted_sorted") {
            read_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { let #name = input.stream.read_compacted_sorted_indices()?; },
              _ => panic!("compacted_sorted used on invalid type {:?}", ty),
            }
          }
        }

        if let Some(assert_eq_val) = assert_eq_val {
          let ident_name = name.to_string();
          read_tokens = quote! {
            #read_tokens
            assert_eq!(#name, #assert_eq_val, "{} is not {}", #ident_name, #assert_eq_val);
          }
        }

        if let Some(conditional_val) = conditional_val {
          read_tokens = quote! {
            let #name = if #conditional_val {
              #read_tokens
              #name
            } else {
              <#ty>::default()
            };
          }
        }

        read_tokens
      }).collect();
      let replay_write_tokens: proc_macro2::TokenStream = punctuated.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let mut assert_eq_val = None;
        let mut conditional_val = None;
        let mut write_tokens = quote! { self.#name.replay_write(w)?; };
        for attribute in &field.attrs {
          if attribute.path().is_ident("assert_eq") {
            assert_eq_val = Some(attribute.parse_args::<Expr>().unwrap());
          } else if attribute.path().is_ident("conditional_or_default") {
            conditional_val = Some(attribute.parse_args::<Expr>().unwrap());
          } else if attribute.path().is_ident("space_optimized") {
            write_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.is_ident("u16") => quote! { w.stream.write_opt_u16(self.#name)?; },
              syn::Type::Path(path_type) if path_type.path.is_ident("u32") => quote! { w.stream.write_opt_u32(self.#name)?; },
              syn::Type::Path(path_type) if path_type.path.is_ident("u64") => quote! { w.stream.write_opt_u64(self.#name)?; },
              _ => panic!("space_optimized used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("vec_u8") {
            write_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { crate::replay::replay_write_vec_u8(&self.#name, w)?; },
              _ => panic!("vec_u8 used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("vec_u16") {
            write_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { crate::replay::replay_write_vec_u16(&self.#name, w)?; },
              _ => panic!("vec_u16 used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("vec_opt_u16") {
            write_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { crate::replay::replay_write_vec_opt_u16(&self.#name, w)?; },
              _ => panic!("vec_u16 used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("vec_u32") {
            write_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { crate::replay::replay_write_vec_u32(&self.#name, w)?; },
              _ => panic!("vec_u32 used on invalid type {:?}", ty),
            }
          } else if attribute.path().is_ident("compacted_sorted") {
            write_tokens = match ty {
              syn::Type::Path(path_type) if path_type.path.segments.len() == 1 && path_type.path.segments.first().unwrap().ident == "Vec" => quote! { w.stream.write_compacted_sorted_indices(&self.#name)?; },
              _ => panic!("compacted_sorted used on invalid type {:?}", ty),
            }
          }
        }

        if let Some(assert_eq_val) = assert_eq_val {
          let ident_name = name.to_string();
          write_tokens = quote! {
            assert_eq!(self.#name, #assert_eq_val, "{} is not {}", #ident_name, #assert_eq_val);
            #write_tokens
          }
        }

        if let Some(conditional_val) = conditional_val {
          write_tokens = quote! {
            if self.#conditional_val {
              #write_tokens
            }
          }
        }

        write_tokens
      }).collect();
      let struct_param_tokens: proc_macro2::TokenStream = punctuated.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        quote! { #name, }
      }).collect();

      let name = &input.ident;

      let expanded = quote! {
        impl #impl_generics crate::replay::ReplayReadWrite for #name #ty_generics #where_clause {
          fn replay_read<R: std::io::BufRead + std::io::Seek>(input: &mut crate::replay::ReplayDeserialiser<R>) -> crate::Result<Self> {
            #replay_read_tokens
            Ok(#name { #struct_param_tokens })
          }
          fn replay_write(&self, w: &mut crate::replay::ReplaySerialiser) -> crate::Result<()> {
            #replay_write_tokens
            Ok(())
          }
        }
      };

      TokenStream::from(expanded)
    },
    syn::Data::Enum(_) => panic!("Can't use ReplayReadWriteStruct on enum type {}.", input.ident),
    syn::Data::Union(_) => panic!("Can't use ReplayReadWriteStruct on union type {}.", input.ident),
  }
}
