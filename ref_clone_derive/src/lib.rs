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
            let quote = quote! {
                #vis fn #fn_ident<'a, T: ::ref_clone::Ref<'a, Self, #ty>>(this: T) -> <T as ::ref_clone::HKT<#ty>>::To {
                    match this.ty() {
                        false => ::ref_clone::Borrow(&this.to_borrow().#ident),
                        true => ::ref_clone::BorrowMut(unsafe { (&this.to_borrow().#ident as *const #ty as *mut #ty).as_ref().unwrap()}),
                    }
                }
            };
            quote
        });
        let gen = quote! {
            impl #name {
                #(#interior)*
            }
        };
        println!("{}", gen);
        gen.into()
    } else {
        panic!("Can not use RefAccessors on a non-strict type.");
    }
}
