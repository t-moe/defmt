use defmt_parser::{Level, ParserMode};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use proc_macro_error2::abort;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Expr, ExprMethodCall};

use crate::construct;

use self::env_filter::EnvFilter;
pub(crate) use self::{args::Args, codegen::Codegen};

mod args;
mod codegen;
mod env_filter;

pub(crate) fn expand(level: Level, args: TokenStream) -> TokenStream {
    expand_parsed(level, parse_macro_input!(args as Args)).into()
}

pub(crate) fn expand_parsed(level: Level, args: Args) -> TokenStream2 {
    let format_string = args.format_string.value();

    {
        //let args = args.formatting_args;
        /*let args = args.map(|punctuated| {
            punctuated.iter().map(|arg|
                quote!(defmt::Format2Debug(&#arg))

            ).collect::<Vec<_>>()
        }).map(|args| {
            quote!(#(#args),*)
        });*/

        /*let level = format_ident!("{}", level.as_str());
        return quote! {
            log::#level!(#format_string, #args);
        };*/
    };

    let fragments = match defmt_parser::parse(&format_string, ParserMode::Strict) {
        Ok(args) => args,
        Err(e) => abort!(args.format_string, "{}", e),
    };

    let formatting_exprs = args
        .formatting_args
        .map(|punctuated| punctuated.into_iter().collect::<Vec<_>>())
        .unwrap_or_default();

    let Codegen { patterns, exprs } = Codegen::new(
        &fragments,
        formatting_exprs.len(),
        args.format_string.span(),
    );

    //let header = construct::interned_string(&format_string, level.as_str(), true, Some(level.as_str()));
    let env_filter = EnvFilter::from_env_var();

    if let Some(filter_check) = env_filter.path_check(level) {
        let level = format_ident!("{}", level.as_str());
        quote!(
            match (#(&(#formatting_exprs)),*) {
                (#(#patterns),*) => {
                    if #filter_check {
                        /*// safety: will be released a few lines further down
                        unsafe { defmt::export::acquire() };
                        defmt::export::header(&#header);
                        #(#exprs;)*
                        // safety: acquire() was called a few lines above
                        unsafe { defmt::export::release() }*/

                        log::#level!(#format_string, #(#exprs),*);

                    }
                }
            }
        )
    } else {
        // if logging is disabled match args, so they are not considered "unused"
        quote!(
            match (#(&(#formatting_exprs)),*) {
                _ => {}
            }
        )
    }
}
