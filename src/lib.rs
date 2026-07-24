//! Compile-time bit-width checks for enum discriminants.
//!
//! `repr_fits` provides an attribute macro for enums whose discriminants are packed into a fixed
//! number of bits in another representation, such as a protocol field or compact integer id.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Fields, Ident, ItemEnum, LitInt, Result, Token, parse_macro_input};

struct ReprFitsArgs {
    bits: u32,
}

impl Parse for ReprFitsArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        if name != "bits" {
            return Err(Error::new(name.span(), "expected `bits = N`"));
        }

        input.parse::<Token![=]>()?;
        let bits = input.parse::<LitInt>()?.base10_parse::<u32>()?;
        if bits >= 128 {
            return Err(Error::new(name.span(), "`bits` must be less than 128"));
        }

        Ok(Self { bits })
    }
}

/// Assert at compile time that each enum variant discriminant fits in `bits` bits.
///
/// The macro preserves the enum definition and appends a private `const` assertion block. It is
/// intended for fieldless enums with primitive integer representations such as `#[repr(u8)]`.
///
/// # Example
///
/// ```rust
/// use repr_fits::repr_fits;
///
/// #[repr_fits(bits = 5)]
/// #[repr(u8)]
/// enum RegionCode {
///     Local = 0,
///     Backup = 4,
/// }
/// ```
///
/// A variant outside the declared bit width fails at compile time:
///
/// ```compile_fail
/// use repr_fits::repr_fits;
///
/// #[repr_fits(bits = 2)]
/// #[repr(u8)]
/// enum PacketKind {
///     Data = 0,
///     Ack = 1,
///     Control = 4,
/// }
/// ```
#[proc_macro_attribute]
pub fn repr_fits(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ReprFitsArgs);
    let item = parse_macro_input!(input as ItemEnum);

    let enum_ident = &item.ident;
    let mut variant_idents = Vec::with_capacity(item.variants.len());
    let mut errors = Vec::new();

    for variant in &item.variants {
        match variant.fields {
            Fields::Unit => variant_idents.push(&variant.ident),
            _ => errors.push(Error::new_spanned(variant, "`repr_fits` only supports unit enum variants")),
        }
    }

    if !errors.is_empty() {
        let compile_errors = errors.into_iter().map(Error::into_compile_error);
        return quote! {
            #item
            #(#compile_errors)*
        }
        .into();
    }

    let bits = args.bits;
    let assertions = variant_idents.iter().map(|variant_ident| {
        let message = format!("{enum_ident}::{variant_ident} discriminant does not fit in {bits} bits");
        quote! {
            assert!((#enum_ident::#variant_ident as u128) < (1u128 << #bits), #message);
        }
    });

    quote! {
        #item

        const _: () = {
            #(#assertions)*
        };
    }
    .into()
}
