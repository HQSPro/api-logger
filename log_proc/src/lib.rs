
use std::sync::atomic::AtomicU32;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemConst, Expr, Lit};

static EXIST_LOG_MASK: AtomicU32 = AtomicU32::new(0);

#[proc_macro_attribute]
pub fn log_mask(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let const_item = parse_macro_input!(item as ItemConst);

    let const_value = const_item.expr.to_owned();
    let const_u32 = match const_value.as_ref() {
        Expr::Lit(v) => match &v.lit {
            Lit::Int(li) => match li.base10_parse::<u32>() {
                Ok(uint) => uint,
                Err(e) => {return e.to_compile_error().into();}
            },
            _ => {return syn::Error::new_spanned(&v, "Must be a number literal.").to_compile_error().into();}
        },
        _ => {return syn::Error::new_spanned(&const_value, "Must be a number literal.").to_compile_error().into();}
    };
    let mut history = EXIST_LOG_MASK.load(std::sync::atomic::Ordering::SeqCst);
    loop {
        let old_history = history;
        let new_history = old_history | const_u32;
        if old_history == new_history {
            return syn::Error::new_spanned(&const_value, "The number must not be the same as any previous value across crates.").to_compile_error().into();
        }
        match EXIST_LOG_MASK.compare_exchange_weak(old_history, new_history, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst) {
            Ok(_) => {
                return const_item.to_token_stream().into();
            },
            Err(_) => {
                history = EXIST_LOG_MASK.load(std::sync::atomic::Ordering::SeqCst);
                continue;
            },
        }
    }
}

