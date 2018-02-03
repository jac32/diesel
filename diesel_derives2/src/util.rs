use syn::*;
use quote::Tokens;
use proc_macro2::Span;

pub use diagnostic_shim::*;

pub fn wrap_in_dummy_mod(const_name: Ident, item: Tokens) -> Tokens {
    quote! {
        mod #const_name {
            // https://github.com/rust-lang/rust/issues/47314
            extern crate std;

            mod diesel {
                __diesel_use_everything!();
            }
            #item
        }
    }
}

pub fn inner_of_option_ty(ty: &Type) -> &Type {
    option_ty_arg(ty).unwrap_or(ty)
}

pub fn is_option_ty(ty: &Type) -> bool {
    option_ty_arg(ty).is_some()
}

fn option_ty_arg(ty: &Type) -> Option<&Type> {
    use syn::PathArguments::AngleBracketed;

    match *ty {
        Type::Path(ref ty) => {
            let last_segment = ty.path.segments.iter().last().unwrap();
            match last_segment.arguments {
                AngleBracketed(ref args) if last_segment.ident == "Option" => {
                    match args.args.iter().last() {
                        Some(&GenericArgument::Type(ref ty)) => Some(ty),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn fix_span(maybe_bad_span: Span, fallback: Span) -> Span {
    let bad_span_debug = "Span(Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 })";
    if format!("{:?}", maybe_bad_span) == bad_span_debug {
        fallback
    } else {
        maybe_bad_span
    }
}
