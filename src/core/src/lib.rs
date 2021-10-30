use quote::quote;
use syn;

//macro_rules!

#[proc_macro_attribute]
pub fn launch(_attr: proc_macro2::TokenStream, item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    // We want to start the server here.
    // This call should have a macro that contains all the necessary information to start all the
    // sockets
    let input = syn::parse2(item);
    let syn::ItemFn { attrs, vis, sig, block } = input;
    let stmts = block.stmts;
    let x =
    quote!(
        fn main() {
            println!("We are the world!");
            #(#stmts)*
        }
    );
    TokenStream::from(x)
}
