extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitInt};
use syn::{meta::ParseNestedMeta, Data};
use syn::parse::{Parse, Result};

#[derive(Default)]
struct CommandAttributes {
    carriage_returns: Option<LitInt>,
    payload_size: Option<LitInt>
}

impl CommandAttributes {
    fn parse(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("carriage_returns") {
            self.carriage_returns = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("payload_size") {
            self.payload_size = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported command property"))
        }
    }
}

#[proc_macro_derive(Command)]
pub fn derive_command(item: TokenStream) -> TokenStream {
    // Parse the arguments/attribtes supplied to the macro, i.e. "carriage_returns=0";
    let mut attrs = CommandAttributes::default();
    let command_parser = syn::meta::parser(|meta| attrs.parse(meta));
    let attr_item = item.clone();
    parse_macro_input!(attr_item with command_parser);
    
    // Parse the struct info.
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_identifier = &input.ident;

    // Generate our base command trait implementation.
    let command_impl = if let Some(carriage_returns) = attrs.carriage_returns {
        quote! {
            use crate::at::commands::Command;
            
            #[automatically_derived]
            impl crate::at::commands::Command for #struct_identifier {
                fn identifier(&self) -> crate::at::commands::Identifier {
                    crate::at::commands::Identifier::#struct_identifier
                }

                fn carriage_returns(&self) -> u8 {
                    #carriage_returns
                }
            }
        }
    } else {
        quote! {
            use crate::at::commands::Command;
            
            #[automatically_derived]
            impl crate::at::commands::Command for #struct_identifier {
                fn identifier(&self) -> crate::at::commands::Identifier {
                    crate::at::commands::Identifier::#struct_identifier
                }
            }
        }
    };

    // Generate our base command trait implementation.
    let from_impl = if let Some(payload_size) = attrs.payload_size {
        quote! {
            #[automatically_derived]
            impl From<#struct_identifier> for crate::at::Command<#payload_size> {
                fn from(cmd: #struct_identifier) -> crate::at::Command<#payload_size> {
                    crate::at::Command{
                        identifier: cmd.identifier(),
                        payload: None,
                        carriage_returns: cmd.carriage_returns(),
                    }
                }
            }
        }
    } else {
        quote! {
            #[automatically_derived]
            impl From<#struct_identifier> for crate::at::Command<0> {
                fn from(cmd: #struct_identifier) -> crate::at::Command<0> {
                    crate::at::Command{
                        identifier: cmd.identifier(),
                        payload: None,
                        carriage_returns: cmd.carriage_returns(),
                    }
                }
            }
        }
    };

    

    // Generate the command implementation code.
    match &input.data {
        Data::Struct(syn::DataStruct { .. }) => {
            quote! {
                #command_impl
                #from_impl
            }
        }
        _ => unimplemented!()
    }.into()
}
