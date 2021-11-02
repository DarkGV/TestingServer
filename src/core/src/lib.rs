extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

//macro_rules!

#[proc_macro_attribute]
pub fn launch(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // We want to start the server here.
    // This call should have a macro that contains all the necessary information to start all the
    // sockets
    let input: syn::ItemFn = syn::parse(item).unwrap();
    let stmts = input.block.stmts;
    quote!(
        fn main() {
            println!("ItemFn ");
            #(#stmts)*
        }
    ).into()
}
