extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::Meta;
use syn::NestedMeta;

#[proc_macro_attribute]
pub fn guard(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    assert!(!args.is_empty() && args.len() < 4);
    let scope = match args.get(0) {
        Some(NestedMeta::Meta(Meta::Path(x))) => {
            if let Some(scope) = x.get_ident() {
                Some(quote! {
                    use ::dino_park_trust::Trust;
                    use ::dino_park_trust::TrustError;

                    if __sau.scope < Trust::#scope {
                        return Err(TrustError::TrustLevelToLow.into());
                    }
                })
            } else {
                panic!();
            }
        }
        _ => panic!(),
    };
    let groups_scope = match args.get(1) {
        Some(NestedMeta::Meta(Meta::Path(x))) => {
            if let Some(groups_scope) = x.get_ident() {
                Some(quote! {
                use ::dino_park_trust::GroupsTrust;
                use ::dino_park_trust::GroupsTrustError;

                if __sau.groups_scope < GroupsTrust::#groups_scope {
                    return Err(GroupsTrustError::GroupsTrustLevelToLow.into());
                }
                })
            } else {
                panic!();
            }
        }
        _ => None,
    };
    let aa_level = match args.get(2) {
        Some(NestedMeta::Meta(Meta::Path(x))) => {
            if let Some(aa_level) = x.get_ident() {
                Some(quote! {
                    use ::dino_park_trust::AALevel;
                    use ::dino_park_trust::AALevelError;

                    if __sau.aa_level < AALevel::#aa_level {
                        return Err(AALevelError::AALevelToLow.into());
                    }
                })
            } else {
                panic!();
            }
        }
        _ => None,
    };

    let mut checks = quote! { #scope };
    if let Some(groups_scope) = groups_scope {
        checks = quote! {
            #checks
            #groups_scope
        };
    }
    if let Some(aa_level) = aa_level {
        checks = quote! {
            #checks
            #aa_level
        };
    }

    let mut function = parse_macro_input!(input as syn::ItemFn);
    let arg: syn::FnArg = parse_quote!(__sau: ::dino_park_gate::scope::ScopeAndUser);
    function.sig.inputs.push(arg);
    let block = function.block;
    let b = parse_quote! {
        {
            {
                use ::std::convert::TryFrom;
                #checks

            }
            #block
        }
    };
    function.block = Box::new(b);

    TokenStream::from(quote!(#function))
}
