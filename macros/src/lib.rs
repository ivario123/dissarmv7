use std::collections::HashMap;

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, Ident, Token};

struct Mask {
    /// The fields to mask out.
    fields: HashMap<char, (usize, Option<usize>)>,
    ident: Ident,
}
impl Parse for Mask {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        println!("Input {}", input);
        let _: Token![=>] = input.parse()?;
        println!("Input {}", input.to_string());
        let span = input.span();
        let input_string = input.to_string();
        let _: Expr = input.parse()?;
        let ignored = [' ', '|'];

        let input_string = input_string
            .chars()
            .filter(|el| !ignored.contains(el))
            .collect::<String>();
        println!("Input {}", input_string);
        if input_string.len() != 32 {
            return Err(syn::Error::new(
                span,
                format!("Expected input of length 32, found {}", input_string.len()),
            ));
        }

        let mut fields: HashMap<char, (usize, Option<usize>)> = HashMap::new();

        for (idx, char) in input_string.char_indices() {
            let idx = match 31usize.checked_sub(idx) {
                Some(val) => val,
                None => {
                    return Err(syn::Error::new(span, "Input is longer than 32 bits"));
                }
            };
            println!("{idx} => {char}");
            if char == 'x' {
                continue;
            }
            let value = fields.get_mut(&char);
            match value {
                Some((start, None)) => {
                    let start = start.clone();
                    let _ = fields.insert(char, (start, Some(idx)));
                }
                None => {
                    let _ = fields.insert(char, (idx, None));
                }
                Some((_start, Some(end))) => {
                    println!("{idx} => {char} {end}");
                    if *end != idx + 1 {
                        return Err(syn::Error::new(
                            span,
                            format!("Field identifier {} is not contiguous", char),
                        ));
                    }
                    *end = idx;
                }
            }
        }
        Ok(Self { fields, ident })
    }
}

#[proc_macro]
pub fn extract_fields(input: TokenStream) -> TokenStream {
    let mask = parse_macro_input!(input as Mask);
    println!("Constructed mask!");
    let mut idents = Vec::with_capacity(mask.fields.len());
    let mut zero = Vec::with_capacity(mask.fields.len());
    let mut mask_calls = Vec::with_capacity(mask.fields.len());
    let mut ret_calls = Vec::with_capacity(mask.fields.len());
    let ret = mask
        .fields
        .iter()
        .map(|_| quote!(u32))
        .collect::<Vec<proc_macro2::TokenStream>>();
    for (key, (start, end)) in mask.fields.iter() {
        let key = syn::Ident::new(&format!("ident_{key}"), Span::call_site().into());
        let end = end.unwrap_or(*start);
        idents.push(quote!(pub #key: u32));
        ret_calls.push(quote!(#key));
        zero.push(quote!(#key: 0));
        mask_calls.push(quote!(self.#key = Self::mask::<#end, #start>(value)));
    }

    let ident = mask.ident;
    let ret = quote! {
        {
            struct Parsed {
                #(#idents,)*
            }

            impl Parsed  {
                const fn mask<const START: usize, const END: usize>(val:u32) -> u32 {
                    let intermediate = val >> START;
                    let mask = ((1 << (END - START + 1) as u32) as u32) - 1u32;

                    let ret = intermediate & mask;
                    assert!(ret <= mask);
                    ret
                }
                const fn parse(mut self,value:u32) -> (#(#ret),*) {
                    #(#mask_calls;)*
                    (#(self.#ret_calls),*)
                }
                const fn zero() -> Self {
                    Self {
                        #(#zero,)*
                    }
                }
            }
            Parsed::zero().parse(#ident)
        }
    };
    println!("Ret : {}", ret.to_string());
    ret.into()
}

struct Comparison {
    ident: Ident,
    mask: u32,
    expected: u32,
}

impl Parse for Comparison {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let _: Token![==] = input.parse()?;
        let str_input = input.to_string();
        let span = input.span();
        let _: syn::Expr = input.parse()?;
        const IGNORED: [char; 2] = [' ', '|'];

        let str_input = str_input
            .chars()
            .filter(|el| !IGNORED.contains(el))
            .collect::<String>();
        let mut mask = 0;
        let mut expected = 0;
        for c in str_input.chars() {
            mask <<= 1;
            expected <<= 1;
            match c {
                '1' => {
                    mask |= 1;
                    expected |= 1;
                }
                '0' => mask |= 1,
                'x' => {}
                _ => {
                    return Err(syn::Error::new(
                        span,
                        &format!("Expected [1,0,x] found {c}"),
                    ))
                }
            }
        }
        Ok(Self {
            ident,
            mask,
            expected,
        })
    }
}
#[proc_macro]
pub fn compare(input: TokenStream) -> TokenStream {
    let comparison = parse_macro_input!(input as Comparison);
    let mask = comparison.mask;
    let expected = comparison.expected;
    let ident = comparison.ident;
    quote! {
        #ident&#mask == #expected
    }
    .into()
}
