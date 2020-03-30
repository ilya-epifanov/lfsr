#![recursion_limit = "128"]

extern crate proc_macro;

use itertools::Itertools;
use lfsr_base::LFSR;
use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Span};
use quote::quote;
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::IntSuffix;
use syn::Token;

#[derive(Debug)]
struct SearchingLFSRLookupInput {
    name: String,
    lfsr_ty: syn::TypePath,
    min_value: u32,
    max_value: u32,
    step: u32,
}

impl Parse for SearchingLFSRLookupInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();
        input.parse::<Token![,]>()?;

        let lfsr_ty = input.parse::<syn::TypePath>()?;
        input.parse::<Token![,]>()?;

        let min_value = input.parse::<syn::LitInt>()?.value() as u32;
        input.parse::<Token![,]>()?;

        let max_value = input.parse::<syn::LitInt>()?.value() as u32;
        input.parse::<Token![,]>()?;

        let step = input.parse::<syn::LitInt>()?.value() as u32;

        Ok(Self {
            name,
            lfsr_ty,
            min_value,
            max_value,
            step,
        })
    }
}

#[proc_macro]
pub fn searching_lfsr_lookup(input: TokenStream) -> TokenStream {
    let input: SearchingLFSRLookupInput = parse_macro_input!(input as SearchingLFSRLookupInput);
    let name = syn::Ident::new(&input.name, Span::call_site());
    let lfsr_ident = &input.lfsr_ty;

    let _tests_mod_name = syn::Ident::new(
        format!("tests_{}", &input.name.to_lowercase()).as_str(),
        Span::call_site(),
    );

    let mut lfsr: Box<dyn LFSR> = {
        match lfsr_ident
            .path
            .segments
            .last()
            .unwrap()
            .value()
            .ident
            .to_string()
            .as_str()
        {
            "Galois32" => Box::new(lfsr_instances::galois::Galois32::default()),
            "Galois31" => Box::new(lfsr_instances::galois::Galois31::default()),
            "Galois30" => Box::new(lfsr_instances::galois::Galois30::default()),
            "Galois29" => Box::new(lfsr_instances::galois::Galois29::default()),
            "Galois28" => Box::new(lfsr_instances::galois::Galois28::default()),
            "Galois27" => Box::new(lfsr_instances::galois::Galois27::default()),
            "Galois26" => Box::new(lfsr_instances::galois::Galois26::default()),
            "Galois25" => Box::new(lfsr_instances::galois::Galois25::default()),
            "Galois24" => Box::new(lfsr_instances::galois::Galois24::default()),
            "Galois23" => Box::new(lfsr_instances::galois::Galois23::default()),
            "Galois22" => Box::new(lfsr_instances::galois::Galois22::default()),
            "Galois21" => Box::new(lfsr_instances::galois::Galois21::default()),
            "Galois20" => Box::new(lfsr_instances::galois::Galois20::default()),
            "Galois19" => Box::new(lfsr_instances::galois::Galois19::default()),
            "Galois18" => Box::new(lfsr_instances::galois::Galois18::default()),
            "Galois17" => Box::new(lfsr_instances::galois::Galois17::default()),
            "Galois16" => Box::new(lfsr_instances::galois::Galois16::default()),
            "Galois15" => Box::new(lfsr_instances::galois::Galois15::default()),
            "Galois14" => Box::new(lfsr_instances::galois::Galois14::default()),
            "Galois13" => Box::new(lfsr_instances::galois::Galois13::default()),
            "Galois12" => Box::new(lfsr_instances::galois::Galois12::default()),
            "Galois11" => Box::new(lfsr_instances::galois::Galois11::default()),
            "Galois10" => Box::new(lfsr_instances::galois::Galois10::default()),
            "Galois9" => Box::new(lfsr_instances::galois::Galois9::default()),
            "Galois8" => Box::new(lfsr_instances::galois::Galois8::default()),
            "Galois7" => Box::new(lfsr_instances::galois::Galois7::default()),
            "Galois6" => Box::new(lfsr_instances::galois::Galois6::default()),
            "Galois5" => Box::new(lfsr_instances::galois::Galois5::default()),
            "Galois4" => Box::new(lfsr_instances::galois::Galois4::default()),
            "Galois3" => Box::new(lfsr_instances::galois::Galois3::default()),
            "Galois2" => Box::new(lfsr_instances::galois::Galois2::default()),
            _ => panic!(),
        }
    };

    let starting_values = (input.min_value..input.max_value)
        .step_by(input.step as usize)
        .collect_vec();
    let mut counter = 0;
    let mut lfsr_states = Vec::new();
    for starting_value in starting_values.iter().cloned() {
        while counter < starting_value {
            lfsr.inc();
            counter += 1;
        }
        lfsr_states.push(lfsr.get_state());
    }

    let step = syn::LitInt::new(input.step as u64, IntSuffix::U32, Span::call_site());
    let steps = lfsr_states.len();

    let lookup_table = {
        let mut s = quote! {};
        for lfsr_state in lfsr_states.iter().cloned() {
            let lfsr_state = syn::LitInt::new(lfsr_state as u64, IntSuffix::U32, Span::call_site());
            s.append_all(quote! {
                #lfsr_state,
            });
        }
        Group::new(Delimiter::Bracket, s)
    };

    let lookups = {
        let mut s = quote! {};
        for (ix, starting_value) in starting_values.iter().cloned().enumerate() {
            let ix = syn::LitInt::new(ix as u64, IntSuffix::Usize, Span::call_site());
            let starting_value =
                syn::LitInt::new(starting_value as u64, IntSuffix::U32, Span::call_site());
            s.append_all(quote! {
                if lfsr == LOOKUP[#ix] {
                    return Some(#starting_value + offset);
                }
            });
        }
        s
    };

    let expanded = quote! {
        fn #name(lfsr: &#lfsr_ident) -> Option<u32> {
            const LOOKUP: [u32; #steps as usize] = #lookup_table;

            let mut lfsr = lfsr.state;

            for offset in 0u32..#step {
                #lookups
                lfsr = #lfsr_ident::down(lfsr);
            }
            None
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug)]
struct DirectLFSRLookupInput {
    name: String,
    lfsr_ty: syn::TypePath,
}

impl Parse for DirectLFSRLookupInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();
        input.parse::<Token![,]>()?;

        let lfsr_ty = input.parse::<syn::TypePath>()?;

        Ok(Self {
            name,
            lfsr_ty,
        })
    }
}

#[proc_macro]
pub fn direct_lfsr_lookup(input: TokenStream) -> TokenStream {
    let input: DirectLFSRLookupInput = parse_macro_input!(input as DirectLFSRLookupInput);
    let name = syn::Ident::new(&input.name, Span::call_site());
    let lfsr_ident = &input.lfsr_ty;

    let _tests_mod_name = syn::Ident::new(
        format!("tests_{}", &input.name.to_lowercase()).as_str(),
        Span::call_site(),
    );

    let mut lfsr: Box<dyn LFSR> = {
        match lfsr_ident
            .path
            .segments
            .last()
            .unwrap()
            .value()
            .ident
            .to_string()
            .as_str()
        {
            "Galois32" => Box::new(lfsr_instances::galois::Galois32::default()),
            "Galois31" => Box::new(lfsr_instances::galois::Galois31::default()),
            "Galois30" => Box::new(lfsr_instances::galois::Galois30::default()),
            "Galois29" => Box::new(lfsr_instances::galois::Galois29::default()),
            "Galois28" => Box::new(lfsr_instances::galois::Galois28::default()),
            "Galois27" => Box::new(lfsr_instances::galois::Galois27::default()),
            "Galois26" => Box::new(lfsr_instances::galois::Galois26::default()),
            "Galois25" => Box::new(lfsr_instances::galois::Galois25::default()),
            "Galois24" => Box::new(lfsr_instances::galois::Galois24::default()),
            "Galois23" => Box::new(lfsr_instances::galois::Galois23::default()),
            "Galois22" => Box::new(lfsr_instances::galois::Galois22::default()),
            "Galois21" => Box::new(lfsr_instances::galois::Galois21::default()),
            "Galois20" => Box::new(lfsr_instances::galois::Galois20::default()),
            "Galois19" => Box::new(lfsr_instances::galois::Galois19::default()),
            "Galois18" => Box::new(lfsr_instances::galois::Galois18::default()),
            "Galois17" => Box::new(lfsr_instances::galois::Galois17::default()),
            "Galois16" => Box::new(lfsr_instances::galois::Galois16::default()),
            "Galois15" => Box::new(lfsr_instances::galois::Galois15::default()),
            "Galois14" => Box::new(lfsr_instances::galois::Galois14::default()),
            "Galois13" => Box::new(lfsr_instances::galois::Galois13::default()),
            "Galois12" => Box::new(lfsr_instances::galois::Galois12::default()),
            "Galois11" => Box::new(lfsr_instances::galois::Galois11::default()),
            "Galois10" => Box::new(lfsr_instances::galois::Galois10::default()),
            "Galois9" => Box::new(lfsr_instances::galois::Galois9::default()),
            "Galois8" => Box::new(lfsr_instances::galois::Galois8::default()),
            "Galois7" => Box::new(lfsr_instances::galois::Galois7::default()),
            "Galois6" => Box::new(lfsr_instances::galois::Galois6::default()),
            "Galois5" => Box::new(lfsr_instances::galois::Galois5::default()),
            "Galois4" => Box::new(lfsr_instances::galois::Galois4::default()),
            "Galois3" => Box::new(lfsr_instances::galois::Galois3::default()),
            "Galois2" => Box::new(lfsr_instances::galois::Galois2::default()),
            _ => panic!(),
        }
    };

    let lfsr_initial_state = lfsr.get_state();

    let mut counter = 0;
    let mut lfsr_states = Vec::new();
    while counter < lfsr.sequence_length() {
        if counter != 0 {
            assert_ne!(lfsr.get_state(), lfsr_initial_state);
        }
        lfsr_states.push(lfsr.get_state());
        lfsr.inc();
        counter += 1;
    }

    let mut reverse_lfsr_states = Vec::with_capacity(lfsr_states.len());
    reverse_lfsr_states.resize_with(lfsr_states.len() + 1, || 0);

    for (ix, lfsr_state) in lfsr_states.iter().copied().enumerate() {
        reverse_lfsr_states[lfsr_state as usize] = ix;
    }

    let steps = lfsr_states.len() + 1;

    let lookup_table = {
        let mut s = quote! {};
        for original_value in reverse_lfsr_states.iter().cloned() {
            let original_value = syn::LitInt::new(original_value as u64, IntSuffix::U32, Span::call_site());
            s.append_all(quote! {
                #original_value,
            });
        }
        Group::new(Delimiter::Bracket, s)
    };

    let expanded = quote! {
        fn #name(lfsr: &#lfsr_ident) -> u32 {
            const LOOKUP: [u32; #steps as usize] = #lookup_table;

            LOOKUP[lfsr.state as usize]
        }
    };

    TokenStream::from(expanded)
}

#[deprecated(since="0.3.0", note="use searching_lfsr_lookup!(..) instead; or use direct_lfsr_lookup!(..) for small LUTs")]
#[proc_macro]
pub fn lfsr_lookup(input: TokenStream) -> TokenStream {
    searching_lfsr_lookup(input)
}
