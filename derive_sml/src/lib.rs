use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields,
};

#[proc_macro_derive(WriteSml)]
pub fn write_sml_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;

    // Generate sml_write_type implementation
    let write_type_impl = quote! {
        fn sml_write_type<W: BitWrite>(writer: &mut W) -> io::Result<()> {
            writer.write(TYPE_IDENT_BIT_SIZE as u32, 7u8)
        }
    };

    // Generate sml_write_value implementation
    let write_value_impl = match &ast.data {
        Data::Struct(data_struct) => {
            let fields = match &data_struct.fields {
                Fields::Named(fields) => &fields.named,
                Fields::Unnamed(fields) => &fields.unnamed,
                Fields::Unit => return TokenStream::from(quote! {
                    compile_error!("WriteSml cannot be derived for unit structs");
                }),
            };

            let field_count = fields.len() as u64;
            let write_field_count = quote! {
                (#field_count as u64).sml_write_value(writer)?;
            };

            let write_fields = fields.iter().enumerate().map(|(i, f)| {
                let field_ident = match &f.ident {
                    Some(ident) => quote! { #ident },
                    None => {
                        let index = syn::Index::from(i);
                        quote! { #index }
                    },
                };
                let ty = &f.ty;
                quote! {
                    <#ty as WriteSml>::sml_write_type(writer)?;
                    self.#field_ident.sml_write_value(writer)?;
                }
            });

            quote! {
                fn sml_write_value<W: BitWrite>(&self, writer: &mut W) -> io::Result<()> {
                    #write_field_count
                    #(#write_fields)*
                    Ok(())
                }
            }
        }
        _ => return TokenStream::from(quote! {
            compile_error!("WriteSml can only be derived for structs");
        }),
    };

    let expanded = quote! {
        impl #generics WriteSml for #name #generics {
            #write_type_impl
            #write_value_impl
        }
    };

    TokenStream::from(expanded)
}