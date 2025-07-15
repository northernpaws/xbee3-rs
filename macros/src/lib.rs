extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitInt, LitStr};
use syn::{meta::ParseNestedMeta, Data};
use syn::parse::{Result};

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

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the arguments/attribtes supplied to the macro, i.e. "carriage_returns=0";
    let mut attrs = CommandAttributes::default();
    let command_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(args with command_parser);
    
    // Parse the struct info.
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

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

    // Generate the command implementation code.
    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut payload_size_bytes: usize = 0;

            // Decoding sequence for the payload type.
            let mut payload_dec: proc_macro2::TokenStream = "None".parse().expect("Failed to parse tokens");

            if let Some(size) = attrs.payload_size {
                // Use the specified payload size if one was provided.
                payload_size_bytes = size.base10_parse()
                    .expect("failed to parse payload size");
            } else {
                match fields {
                    syn::Fields::Named(fields_named) => todo!("automatic decoding of named struct fields is not implemented"),
                    syn::Fields::Unnamed(fields_unnamed) => {
                        if fields_unnamed.unnamed.len() == 1 {
                            let field = fields_unnamed.unnamed.first().expect("needs first field");
                            payload_size_bytes =  type_size_bytes(&field.ty);
                            
                            match &field.ty {
                                syn::Type::Array(type_array) => { // [T; N] -> T::size * N
                                    match type_array.elem.as_ref() {
                                        syn::Type::Verbatim(token_stream) => {
                                            payload_dec = format!("Some({})",token_stream.to_string() ).parse().expect("Failed to parse tokens");
                                        },
                                        _ => todo!("unexpected array element type"),
                                    }
                                },
                                syn::Type::Path(type_path) => {
                                    // If single segment, it's probably a type like u16.
                                    if type_path.path.segments.len() == 1 {
                                        let segment = type_path.path.segments.first().expect("expected path segment");
                                        match segment.ident.to_string().as_str() {
                                            "u8" => {
                                                payload_size_bytes = 3;
                                                payload_dec = "Some(super::u8_ascii(cmd.0 as u8))".parse().expect("Failed to parse tokens");
                                            },
                                            "u16" | "u32" | "u64" => {
                                                // TODO: proper decoding to ascii
                                                payload_dec = "Some(cmd.0.to_be_bytes())".parse().expect("Failed to parse tokens");
                                            },
                                            _ => todo!("cannot determine size of type for payload"),
                                        }
                                    } else {
                                        todo!("cannot determine size of path for payload {:?}", type_path)
                                    }
                                },
                                syn::Type::Verbatim(token_stream) => {
                                    payload_dec = format!("Some({})",token_stream.to_string() ).parse().expect("Failed to parse tokens");
                                },
                                _ => todo!("cannot automatically determine encoding for payload type: {:?}", &field.ty),
                            }
                        }
                    },
                    syn::Fields::Unit => {},
                }
            };

            let from_impl = quote! {
                #[automatically_derived]
                impl From<#struct_identifier> for crate::at::Command<#payload_size_bytes> {
                    fn from(cmd: #struct_identifier) -> crate::at::Command<#payload_size_bytes> {
                        crate::at::Command{
                            identifier: cmd.identifier(),
                            payload: #payload_dec,
                            carriage_returns: cmd.carriage_returns(),
                        }
                    }
                }
            };

            quote! {
                #input
                #command_impl
                #from_impl
            }
        }
        _ => unimplemented!()
    }.into()
}

/// Returns the size of the input type in bytes.
fn type_size_bytes(ty: &syn::Type) -> usize {
    let mut type_mult = 1;
    
    match &ty {
        syn::Type::Array(type_array) => { // [T; N] -> T::size * N
            match type_array.elem.as_ref() {
                syn::Type::Path(type_path) => todo!("paths are not a valid payload array element type"),
                syn::Type::Verbatim(token_stream) => todo!("verbatim"),
                _ => todo!("unexpected payload array element type: {:?}", type_array.elem),
            }
        },
        syn::Type::BareFn(type_bare_fn) => todo!("barefn"),
        syn::Type::Group(type_group) => todo!("group"),
        syn::Type::ImplTrait(type_impl_trait) => todo!("implrtait"),
        syn::Type::Infer(type_infer) => todo!("infer"),
        syn::Type::Macro(type_macro) => todo!("macro"),
        syn::Type::Never(type_never) => todo!("never"),
        syn::Type::Paren(type_paren) => todo!("paren"),
        syn::Type::Path(type_path) => {
            // If single segment, it's probably a type like u16.
            if type_path.path.segments.len() == 1 {
                let segment = type_path.path.segments.first().expect("expected path segment");
                match segment.ident.to_string().as_str() {
                    "u8" => {
                        type_mult = 1;
                    },
                    "u16" => {
                        type_mult = 2;
                    },
                    "u32" => {
                        type_mult = 4;
                    },
                    "u64" => {
                        type_mult = 8;
                    },
                    _ => todo!("cannot determine size of type for payload"),
                }
            } else {
                todo!("cannot determine size of path for payload {:?}", type_path)
            }
        },
        syn::Type::Ptr(type_ptr) => todo!("ptr"),
        syn::Type::Reference(type_reference) => todo!("ref"),
        syn::Type::Slice(type_slice) => todo!("slice"),
        syn::Type::TraitObject(type_trait_object) => todo!("trait obj"),
        syn::Type::Tuple(type_tuple) => todo!("tuple"),
        syn::Type::Verbatim(token_stream) => todo!("verb"),
        _ => todo!("unexpected type for payload size"),
    }

    type_mult
}