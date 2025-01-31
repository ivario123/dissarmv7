use std::usize;

use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, spanned::Spanned, BinOp, Expr, Ident, Token};

struct Mask {
    /// The fields to mask out.
    fields: Vec<(char, (usize, Option<usize>))>,
    ident: Ident,
}
impl Parse for Mask {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let span = input.span();
        let input_string = input.to_string();
        let _: Expr = input.parse()?;
        let ignored = [' ', '|'];

        let input_string = input_string
            .chars()
            .filter(|el| !ignored.contains(el))
            .collect::<String>();
        if input_string.len() != 32 {
            return Err(syn::Error::new(
                span,
                format!("Expected input of length 32, found {}", input_string.len()),
            ));
        }

        let mut fields: Vec<(char, (usize, Option<usize>))> = Vec::new();

        let mut parsing: Option<(char, (usize, Option<usize>))> = None;
        for (idx, char) in input_string.char_indices() {
            let idx = match 31usize.checked_sub(idx) {
                Some(val) => val,
                None => {
                    return Err(syn::Error::new(span, "Input is longer than 32 bits"));
                }
            };
            if char == 'x' {
                continue;
            }
            match parsing {
                Some((c, (start, None))) if c == char => {
                    parsing = Some((char, (start, Some(idx))));
                }
                None => {
                    parsing = Some((char, (idx, None)));
                }
                Some((c, (start, Some(end)))) if c == char => {
                    if end != idx + 1 {
                        return Err(syn::Error::new(
                            span,
                            format!("Field identifier {} is not contiguous", char),
                        ));
                    }
                    parsing = Some((char, (start, Some(idx))));
                }
                _ => {
                    if let Some(parsing) = parsing {
                        fields.push(parsing);
                    }
                    parsing = Some((char, (idx, None)));
                }
            }
        }
        if let Some(parsed) = parsing {
            fields.push(parsed);
        }

        Ok(Self { fields, ident })
    }
}

#[proc_macro]
pub fn extract_fields(input: TokenStream) -> TokenStream {
    let mask = parse_macro_input!(input as Mask);
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
    ret.into()
}

struct Comparison {
    ident: Ident,
    mask: u32,
    expected: u32,
    op: BinOp,
}

impl Parse for Comparison {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let op: BinOp = input.parse()?;
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
            op,
        })
    }
}
#[proc_macro]
pub fn compare(input: TokenStream) -> TokenStream {
    let comparison = parse_macro_input!(input as Comparison);
    let mask = comparison.mask;
    let expected = comparison.expected;
    let ident = comparison.ident;
    let op = comparison.op;
    let ret = quote! {
        ((#ident&#mask) #op #expected)
    };
    ret.into()
}

struct Combiner {
    args: Vec<(char, (usize, Option<usize>))>,
    identifiers: Vec<Expr>,
    bit_vector: u32,
}

impl Parse for Combiner {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let bit_str_e: Expr = input.parse()?;
        let bit_str = bit_str_e.to_token_stream().to_string();
        const IGNORED: [char; 2] = [' ', '|'];

        let bit_str = bit_str
            .chars()
            .filter(|el| !IGNORED.contains(el))
            .collect::<String>();
        let _: Token![,] = input.parse()?;

        let idents = input
            .parse_terminated(Expr::parse, Token![,])?
            .iter()
            .map(|el| el.clone().into())
            .collect::<Vec<Expr>>();

        let mut accumulator: u32 = 0;
        let mut args: Vec<(char, (usize, Option<usize>))> = Vec::new();
        let mut parsing: Option<(char, (usize, Option<usize>))> = None;
        if bit_str.len() > 32 {
            return Err(syn::Error::new(
                bit_str_e.span(),
                &format!(
                    "Expected a bitstring of 32 elements, got {} elements",
                    bit_str.len()
                ),
            ));
        }
        for (idx, char) in bit_str.chars().enumerate() {
            let idx = 31 - idx;
            accumulator <<= 1;
            accumulator |= match char {
                '1' => 1,
                '0' => 0,
                c => match parsing {
                    Some((c2, (start, Some(end)))) if c2 == c => {
                        if end != idx + 1 {
                            return Err(syn::Error::new(
                                bit_str_e.span(),
                                &format!("{c} is not contiguous"),
                            ));
                        }
                        parsing = Some((c, (start, Some(idx))));
                        0
                    }
                    Some((c2, (start, None))) if c2 == c => {
                        parsing = Some((c, (start, Some(idx))));
                        0
                    }
                    None => {
                        parsing = Some((c, (idx, None)));
                        0
                    }
                    Some(val) => {
                        args.push(val);
                        parsing = Some((c, (idx, None)));
                        0
                    }
                },
            }
        }

        if let Some(val) = parsing {
            args.push(val);
        }
        if idents.len() != args.len() {
            return Err(syn::Error::new(
                bit_str_e.span(),
                &format!("Expected {} arguments got {}", args.len(), idents.len()),
            ));
        }
        Ok(Self {
            args,
            identifiers: idents,
            bit_vector: accumulator,
        })
    }
}

#[proc_macro]
/// Combines a bitstring with in scope variables.
///
/// ```no_run
/// 
/// use macros::combine;
///
/// let a:u32 = 111;
/// let b:u32 = 11;
/// let comb = combine!(1100110xxx111000ccc,a,b);
/// assert!(comb == 1100110111111000011);
/// ```
///
/// The macro will replace chars in the order they occur with the expressions
/// passed in the same order.
pub fn combine(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Combiner);

    let ret: Vec<(&Expr, (usize, usize))> = input
        .args
        .iter()
        .zip(&input.identifiers)
        .map(|((_c, (start, end)), id)| (id, (end.unwrap_or(*start), *start)))
        .collect();

    let masks: Vec<proc_macro2::TokenStream> = ret
        .iter()
        .map(|(id, (start, end))| {
            quote! {ret |= {
                (u32::from(#id) .mask::<0,{#end - #start}>() << #start )
            };}
        })
        .collect();

    let ret = input.bit_vector;
    let ret = quote! {
        {
            let mut ret:u32 = #ret;
            #(#masks)*
            ret
        }
    };

    ret.into()
}

#[proc_macro]
/// Combines a bitstring with in scope variables.
///
/// ```no_run
/// 
/// use macros::combine;
///
/// let a:u32 = 111;
/// let b:u32 = 11;
/// let comb = combine!(1100110xxx111000ccc,a,b);
/// assert!(comb ==     1100110011111000111);
/// ```
///
/// The macro will replace chars in the order they occur with the expressions
/// passed in the same order.
pub fn combine_reverse_order(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Combiner);

    let ret: Vec<(&Expr, (usize, usize))> = input
        .args
        .iter()
        .zip(input.identifiers.iter().rev())
        .map(|((_c, (start, end)), id)| (id, (end.unwrap_or(*start), *start)))
        .collect();

    let masks: Vec<proc_macro2::TokenStream> = ret
        .iter()
        .map(|(id, (start, end))| {
            quote! {ret |= {
                (u32::from(#id) .mask::<0,{#end - #start}>() << #start )
            };}
        })
        .collect();

    let ret = input.bit_vector;
    let ret = quote! {
        {
            let mut ret:u32 = #ret;
            #(#masks)*

            ret
        }
    };

    ret.into()
}
