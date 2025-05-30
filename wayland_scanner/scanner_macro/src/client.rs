use crate::parser;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub trait GenClientTokens {
    fn to_tokens(self) -> TokenStream;
}

impl GenClientTokens for parser::Protocol {
    fn to_tokens(self) -> TokenStream {
        let name = Ident::new(&self.name, Span::call_site());
        let interfaces = self
            .interfaces
            .into_iter()
            .map(parser::Interface::to_tokens)
            .collect::<Vec<TokenStream>>();

        quote! {
            mod #name {
                #( #interfaces )*
            }
            pub use #name::*;
        }
    }
}

impl GenClientTokens for parser::Interface {
    fn to_tokens(self) -> TokenStream {
        let name = Ident::new(&self.name, Span::call_site());
        let type_name = Ident::new(&self.type_name, Span::call_site());
        let requests = self
            .requests
            .into_iter()
            .map(parser::Request::to_tokens)
            .collect::<Vec<TokenStream>>();

        quote! {
            mod #name {
                pub struct #type_name {}

                impl #type_name {
                    #( #requests )*
                }
            }
            pub use #name::*;
        }
    }
}

impl GenClientTokens for parser::Request {
    fn to_tokens(self) -> TokenStream {
        let name = Ident::new(&self.name, Span::call_site());
        let args = self
            .args
            .into_iter()
            .map(parser::Arg::to_tokens)
            .collect::<Vec<TokenStream>>();

        quote! {
            pub fn #name(&self, #( #args ),*) {}
        }
    }
}

impl GenClientTokens for parser::Arg {
    fn to_tokens(self) -> TokenStream {
        let name = Ident::new(&self.name, Span::call_site());
        let type_name = Ident::new("u32", Span::call_site());
        quote! {
            #name: #type_name
        }
    }
}
