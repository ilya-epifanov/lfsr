#![recursion_limit = "128"]

extern crate proc_macro;

use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::IntSuffix;
use syn::Token;

fn galois_mask(taps: &[u32]) -> u32 {
    let mut ret: u32 = 0;
    for tap in taps {
        ret |= 1 << *tap;
    }
    ret
}

fn galois_inverse_mask(forward_mark: u32, width: u32) -> u32 {
    (forward_mark << 1) | ((forward_mark >> (width - 1)) & 1)
}

#[derive(Debug)]
struct LFSRInput {
    name: String,
    width: u32,
    sequence_length: u32,
    taps: Vec<u32>,
}

impl Parse for LFSRInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();
        input.parse::<Token![,]>()?;

        let width = input.parse::<syn::LitInt>()?.value() as u32;
        input.parse::<Token![,]>()?;

        let sequence_length = input.parse::<syn::LitInt>()?.value() as u32;
        input.parse::<Token![,]>()?;

        let taps = input
            .parse_terminated::<syn::LitInt, Token![,]>(syn::LitInt::parse)?
            .iter()
            .map(|l| l.value() as u32)
            .collect_vec();

        Ok(Self {
            name,
            width,
            sequence_length,
            taps,
        })
    }
}

#[proc_macro]
pub fn galois_lfsr(input: TokenStream) -> TokenStream {
    let input: LFSRInput = parse_macro_input!(input as LFSRInput);
    let name = syn::Ident::new(&input.name, Span::call_site());
    let tests_mod_name = syn::Ident::new(
        format!("tests_{}", &input.name.to_lowercase()).as_str(),
        Span::call_site(),
    );
    let width = syn::LitInt::new(input.width as u64, IntSuffix::U32, Span::call_site());
    let sequence_length = syn::LitInt::new(
        input.sequence_length as u64,
        IntSuffix::U32,
        Span::call_site(),
    );

    let taps_str = &input.taps.iter().map(ToString::to_string).join(", ");
    let fwd_mask_u32 = galois_mask(input.taps.iter().map(|t| t - 1).collect_vec().as_slice());
    let fwd_mask = syn::LitInt::new(fwd_mask_u32 as u64, IntSuffix::U32, Span::call_site());
    let inv_mask = syn::LitInt::new(
        galois_inverse_mask(fwd_mask_u32, input.width) as u64,
        IntSuffix::U32,
        Span::call_site(),
    );

    let struct_comment = format!(r#"
        {}-bit [Galois LFSR](https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Galois_LFSRs) with taps at {}.
        Sequence length is {}.
        `0` is a lock-up state.
    "#,
        input.width, taps_str, input.sequence_length);

    let expanded = quote! {
                #[doc=#struct_comment]
                pub struct #name {
                    pub state: u32,
                }

                impl #name {
                    pub const fn new(initial_state: u32) -> Self {
                        Self {
                            state: initial_state,
                        }
                    }

                    const fn sequence_length() -> u32 {
                        #sequence_length
                    }

                    pub const fn up(prev_state: u32) -> u32 {
                        let mut state = prev_state;
                        let lsb = state & 1;
                        state >>= 1;
                        // make Rust happy about absence of conditionals in this const fn
                        state ^= ((lsb.wrapping_neg()) & #fwd_mask);
    //                    if lsb == 1 {
    //                        state ^= #fwd_mask;
    //                    }
                        state
                    }

                    pub const fn down(prev_state: u32) -> u32 {
                        let mut state = prev_state;
                        let msb = state >> (#width - 1);
                        state <<= 1;
                        // make Rust happy about absence of conditionals in this const fn
                        state ^= ((msb.wrapping_neg()) & #inv_mask);
    //                    if msb != 0 {
    //                        state ^= #inv_mask;
    //                    }
                        state
                    }
                }

                /**
                  Default state is `1`
                */
                impl Default for #name {
                    fn default() -> Self {
                        Self::new(1)
                    }
                }

                impl core::fmt::Display for #name {
                    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                        write!(f, "{:032b}", self.get_state())
                    }
                }

                impl lfsr_base::LFSR for #name {
                    fn get_state(&self) -> u32 {
                        self.state
                    }

                    fn inc(&mut self) {
                        self.state = Self::up(self.state);
                    }

                    fn dec(&mut self) {
                        self.state = Self::down(self.state);
                    }
                }

                impl lfsr_base::LFSRStatic for #name {
                    fn sequence_length() -> u32 {
                        Self::sequence_length()
                    }
                }

                #[cfg(test)]
                mod #tests_mod_name {
                    use super::#name;
                    use lfsr_base::LFSR;

                    #[test]
                    fn sequence_length() {
                        let mut f = #name::default();
                        let initial_state = f.state;
                        let mut count = 1;
                        f.inc();
                        while (f.state != initial_state) {
                            f.inc();
                            count += 1;

                            assert!(count <= #name::sequence_length());
                        }

                        assert_eq!(count, #name::sequence_length());
                    }

                    #[test]
                    fn back_and_forth() {
                        let mut f = #name::default();
                        let initial_state = f.state;
                        for _ in 0..1000 {
                            f.inc();
                        }
                        for _ in 0..1000 {
                            f.dec();
                        }
                        assert_eq!(f.state, initial_state);
                    }
                }
            };

    TokenStream::from(expanded)
}
