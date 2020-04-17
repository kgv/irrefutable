use self::irrefutable::{Attribute, Irrefutable, Item};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

/// Irrefutable.
///
/// # Examples
///
/// unreachable:
///
/// ```
/// # #![feature(proc_macro_hygiene)]
/// # use irrefutable::irrefutable;
/// #[irrefutable(unreachable)]
/// let Some((a, b)) = Some(("a", "b"));
/// ```
///
/// return:
///
/// ```
/// # #![feature(proc_macro_hygiene)]
/// # use irrefutable::irrefutable;
/// #[irrefutable(return)]
/// let Some((a, b)) = Some(("a", "b"));
/// ```
/// 
/// #[irrefutable(panic("ashkjsakj"))]
#[proc_macro_attribute]
pub fn irrefutable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Attribute);
    let item = parse_macro_input!(item as Item);
    let irrefutable = Irrefutable { attr, item };
    irrefutable.into_token_stream().into()
}

mod irrefutable;
