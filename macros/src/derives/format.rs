use proc_macro::TokenStream;
use proc_macro_error2::abort_call_site;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

mod codegen;

pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let mut stmts_core = vec![];
    let encode_data = match &input.data {
        Data::Enum(data) => codegen::encode_enum_data(ident, data),
        Data::Struct(data) => {

            stmts_core.extend(data.fields.iter().map(|field| {
                let ident = field.ident.as_ref().unwrap();
                quote!(.field(stringify!(#ident), &self.#ident))
            }));

            codegen::encode_struct_data(ident, data)
        },
        Data::Union(_) => abort_call_site!("`#[derive(Format)]` does not support unions"),
    };

    let codegen::EncodeData {
        format_tag,
        stmts,
        where_predicates,
    } = match encode_data {
        Ok(data) => data,
        Err(e) => return e.into_compile_error().into(),
    };

    let codegen::Generics {
        impl_generics,
        type_generics,
        where_clause,
    } = codegen::Generics::codegen(&mut input.generics, where_predicates);
    let idents = ident.to_string();
    quote!(
        impl #impl_generics defmt::Format for #ident #type_generics #where_clause {
            fn format(&self, f: defmt::Formatter) {
                defmt::unreachable!()
            }

            fn _format_tag() -> defmt::Str {
                #format_tag
            }

            fn _format_data(&self) {
                #(#stmts)*
            }

            fn _core_fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                fmt.debug_struct(#idents)
                #(#stmts_core)*
                .finish()
            }
        }
    )
    .into()
}
