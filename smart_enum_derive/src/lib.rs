extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(SmartEnum)]
pub fn derive_enum_variant_count(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let syn_item = syn::parse_macro_input(&s).unwrap();
    let ast = syn::parse_derive_input(&s).unwrap();
    let variants = match syn_item.body {
        syn::Body::Enum(variants) => variants,
        _ => panic!("Every type other than an enum has exactly 1 (one) variant, please just use that value instead of wasting CPU cycles"),
    };

    //panic!("code: {}", impl_smart_enum(&ast, variants));
    impl_smart_enum(&ast, variants).parse().unwrap()
}

fn impl_smart_enum(ast: &syn::DeriveInput, variants: Vec<syn::Variant>) -> quote::Tokens {
    let name = &ast.ident;
    let len = variants.len();
    let all_values: Vec<_> = variants.iter().map(|v| v.ident.clone()).collect();
    let names: Vec<_> = all_values.iter().map(|_| name.clone()).collect();
    quote! {
        impl ::smart_enum::SmartEnum for #name {
            const LENGTH: usize = #len;
            type ValuesType = ::std::iter::Cloned<::std::slice::Iter<'static, #name>>;

            fn values() -> Self::ValuesType {
                [ #( #names::#all_values ),*].iter().cloned()
            }

            fn as_usize(&self) -> usize {
                self.clone() as usize
            }
        }
    }
}
