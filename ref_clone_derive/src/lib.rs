use syn::Field;
use syn::DataStruct;
use syn::Fields::Named;
use syn::Data::Struct;
use proc_macro::TokenStream;
use quote::quote;
use quote::format_ident;
use syn;

#[proc_macro_derive(RefAccessors)]
pub fn ref_accessors_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_ref_accessors(&ast)
}

fn impl_ref_accessors(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if let Struct(DataStruct { fields: Named(data), .. }) = &ast.data {
        let data = data.named.iter();
        let interior = data.map(|x| {
            let Field { vis, ident, ty, .. } = x;
            let fn_ident = format_ident!("get_{}", ident.as_ref().unwrap());
            let tr_ident = format_ident!("Ref{}{}", name, ident.as_ref().unwrap());
            println!("{}", tr_ident);
            let quote = quote! {
                #vis trait #tr_ident<'a, T> {
                    fn #fn_ident(self) -> Ref<'a, #ty, T>;
                }
                impl<'a, T> #tr_ident<'a, T> for Ref<'a, #name, T> {
                    fn #fn_ident(self) -> Ref<'a, #ty, T> {
                        unsafe { *(&&((*(&self.x as *const _ as *const &#name)).#ident) as *const _ as *const Ref<'a, #ty, T>) }
                    }
                }
            };
            quote
        });
        let gen = quote! {
            #(#interior)*
        };
        println!("{}", gen);
        gen.into()
    } else {
        panic!("Can not use RefAccessors on a non-strict type.");
    }
}
