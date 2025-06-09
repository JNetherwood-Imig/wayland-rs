use crate::parser;
use proc_macro2::{Span, TokenStream};
use quote::{IdentFragment, quote};
use syn::Ident;

pub trait GenClientTokens {
    fn to_tokens(self) -> TokenStream;
}

impl GenClientTokens for parser::Protocol {
    fn to_tokens(self) -> TokenStream {
        let name = Ident::new(&self.name, Span::call_site());
        let interfaces = self
            .interfaces
            .iter()
            .map(|i| i.to_tokens())
            .collect::<Vec<TokenStream>>();

        quote! {
            pub mod #name {
                #( #interfaces )*
            }
        }
    }
}

impl GenClientTokens for &parser::Interface {
    fn to_tokens(self) -> TokenStream {
        let name = Ident::new(&self.name, Span::call_site());
        let type_name = Ident::new(&self.type_name, Span::call_site());
        let requests = self
            .requests
            .iter()
            .map(|r| r.to_tokens())
            .collect::<Vec<TokenStream>>();
        let events = self
            .events
            .iter()
            .map(|e| e.to_tokens())
            .collect::<Vec<TokenStream>>();
        let enums = self
            .enums
            .iter()
            .map(|e| e.to_tokens())
            .collect::<Vec<TokenStream>>();

        quote! {
            pub mod #name {
                pub struct #type_name {}

                impl #type_name {
                    #( #requests )*
                }

                #( #events )*

                #( #enums )*
            }
        }
    }
}

impl GenClientTokens for &parser::Request {
    fn to_tokens(self) -> TokenStream {
        let name = Ident::new_raw(&self.name, Span::call_site());
        let args = self
            .args
            .iter()
            .map(|a| a.to_tokens())
            .collect::<Vec<TokenStream>>();

        quote! {
            pub fn #name(&self, #( #args ),*) {}
        }
    }
}

impl GenClientTokens for &parser::Event {
    fn to_tokens(self) -> TokenStream {
        let mut type_name = self.type_name.clone();
        type_name.push_str("Event");
        let name = Ident::new(&type_name, Span::call_site());
        let args = self
            .args
            .iter()
            .map(|a| a.to_tokens())
            .collect::<Vec<TokenStream>>();

        quote! {
            pub struct #name {
                #( #args ),*
            }
        }
    }
}

impl GenClientTokens for &parser::Enum {
    fn to_tokens(self) -> TokenStream {
        let name = Ident::new(&self.type_name, Span::call_site());
        let entries = self
            .entries
            .iter()
            .map(|e| e.to_tokens())
            .collect::<Vec<TokenStream>>();

        quote! {
            #[repr(u32)]
            pub enum #name {
                #( #entries ),*
            }
        }
    }
}

impl GenClientTokens for &parser::Entry {
    fn to_tokens(self) -> TokenStream {
        let name = Ident::new(&self.valid_name, Span::call_site());
        let value = syn::LitInt::new(self.value.to_string().as_str(), Span::call_site());
        quote! {#name = #value}
    }
}

impl GenClientTokens for &parser::Arg {
    fn to_tokens(self) -> TokenStream {
        let name = Ident::new_raw(&self.name, Span::call_site());
        let type_name = Ident::new("u32", Span::call_site());
        quote! { #name: #type_name }
    }
}
