use self::kw::{panic, unreachable};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, ExprLet, LitStr, Pat, PatBox, PatIdent, PatReference, PatStruct, PatTuple,
    PatTupleStruct, Result, Token,
};

fn parse_binds<'a>(binds: &mut Vec<Bind<'a>>, pat: &'a Pat) {
    match pat {
        Pat::Box(PatBox { pat, .. }) => parse_binds(binds, pat),
        Pat::Ident(PatIdent {
            by_ref,
            mutability,
            ident,
            ..
        }) => binds.push(Bind {
            by_ref,
            mutability,
            ident,
        }),
        Pat::Reference(PatReference { pat, .. }) => parse_binds(binds, pat),
        Pat::Struct(PatStruct { fields, .. }) => {
            for field in fields {
                parse_binds(binds, &field.pat);
            }
        }
        Pat::Tuple(PatTuple { elems, .. })
        | Pat::TupleStruct(PatTupleStruct {
            pat: PatTuple { elems, .. },
            ..
        }) => {
            for elem in elems {
                parse_binds(binds, elem);
            }
        }
        Pat::Lit(_) | Pat::Wild(_) => {}
        _ => {}
    }
}

/// Irrefutable.
pub(super) struct Irrefutable {
    pub(super) attr: Attribute,
    pub(super) item: Item,
}

impl ToTokens for Irrefutable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attr = &self.attr;
        let binds = self.item.binds();
        let inner_binds = binds.iter().map(Kind::Inner);
        let outer_binds = binds.iter().map(Kind::Outer);
        let expr_let = &self.item.expr_let;
        let semi = &self.item.semi_token;
        let attribute_tokens = quote! {
            let (#(#outer_binds),*) = if #expr_let {
                (#(#inner_binds),*)
            } else {
                #attr
            } #semi
        };
        attribute_tokens.to_tokens(tokens);
    }
}

/// Attribute.
pub(super) enum Attribute {
    Panic {
        format: LitStr,
        comma_token: Option<Token![,]>,
        args: Punctuated<Expr, Token![,]>,
    },
    Return,
    Unreachable,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(panic) {
            input.parse::<panic>()?;
            let content;
            let _paren_token = parenthesized!(content in input);
            let format = content.parse::<LitStr>()?;
            let comma_token = content.parse::<Option<Token![,]>>()?;
            let args = if comma_token.is_some() {
                content.parse_terminated(Expr::parse)?
            } else {
                Punctuated::new()
            };
            Ok(Self::Panic {
                format,
                comma_token,
                args,
            })
        } else if lookahead.peek(Token![return]) {
            input.parse::<Token![return]>()?;
            Ok(Self::Return)
        } else if lookahead.peek(unreachable) {
            input.parse::<unreachable>()?;
            Ok(Self::Unreachable)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attribute_tokens = match self {
            Self::Panic {
                format,
                comma_token,
                args,
            } => quote!(panic!(#format #comma_token #args);),
            Self::Return => quote!(return;),
            Self::Unreachable => quote!(unreachable!();),
        };
        attribute_tokens.to_tokens(tokens);
    }
}

/// Item.
pub(super) struct Item {
    expr_let: ExprLet,
    semi_token: Token![;],
}

impl Item {
    fn binds(&self) -> Vec<Bind> {
        let mut binds = Vec::new();
        parse_binds(&mut binds, &self.expr_let.pat);
        binds
    }
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            expr_let: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

#[derive(Clone, Copy)]
struct Bind<'a> {
    by_ref: &'a Option<Token![ref]>,
    mutability: &'a Option<Token![mut]>,
    ident: &'a Ident,
}

enum Kind<'a> {
    Inner(&'a Bind<'a>),
    Outer(&'a Bind<'a>),
}

impl ToTokens for Kind<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let bind_tokens = match self {
            Self::Inner(Bind {
                by_ref,
                mutability: _,
                ident,
            }) => match by_ref {
                Some(_) => quote!(*#ident),
                None => quote!(#ident),
            },
            Self::Outer(Bind {
                by_ref,
                mutability,
                ident,
            }) => quote!(#by_ref #mutability #ident),
        };
        bind_tokens.to_tokens(tokens);
    }
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(panic);
    custom_keyword!(unreachable);
}
