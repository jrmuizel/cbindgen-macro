extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemImpl, ImplItemMethod};
use quote::ToTokens;

fn gen_method(type_name: &syn::Path, method: &ImplItemMethod, tokens: &mut TokenStream) {
    let name = &method.sig.ident;
    let output = &method.sig.decl.output;


    let type_ident = &type_name.segments.last().unwrap().value().ident;

    let concatenated = format!("{}_{}", type_ident, name);

    let mangled_name = syn::Ident::new(&concatenated, name.span());

    // build up the list of arguments to the function we're generating
    let mut args = Vec::new();
    args.push(quote!(t: &mut #type_ident));
    for input in method.sig.decl.inputs.iter().skip(1) {
        args.push(quote!(#input));
    }

    // build up the list arguments to the function we're going to call
    let mut call_args = Vec::new();
    for input in method.sig.decl.inputs.iter().skip(1) {
        match input {
            syn::FnArg::Captured(arg) => { let pat = &arg.pat; call_args.push(quote!(#pat)) }
            _ => { panic!() }
        }
    }

    let result = quote! {
        #[no_mangle]
        pub extern "C" fn #mangled_name(#(#args),*) #output { t.#name(#(#call_args),*) }
    };
    result.to_tokens(tokens);
}

fn gen_destructor(type_name: &syn::Path, tokens: &mut TokenStream)  {

    let type_ident = &type_name.segments.last().unwrap().value().ident;

    let destructor_name = format!("{}_destructor", type_ident);
    let destructor_name = syn::Ident::new(&destructor_name, type_ident.span());

    let destructor = quote! {
        #[no_mangle]
        pub extern "C" fn #destructor_name(t: *mut #type_ident) { t.drop_in_place() }
    };
    destructor.to_tokens(tokens);
}

#[proc_macro_attribute]
pub fn export_c(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemImpl);

    let type_name = match *input.self_ty {
        syn::Type::Path(ref path) => &path.path,
        _ => panic!()
    };


    let mut tokens = TokenStream::new();

    let result = quote! {
        #input
    };
    result.to_tokens(&mut tokens);

    for i in input.items {
        match i {
            syn::ImplItem::Method(ref method) => gen_method(type_name, method, &mut tokens),
            _ => {}
        }
    }


    gen_destructor(type_name, &mut tokens);

    tokens.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
