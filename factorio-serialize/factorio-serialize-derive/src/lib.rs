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
