extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn repeat(items: TokenStream) -> TokenStream {
    let string = items.to_string();
    let items: Vec<&str> = string.split("for_ ").collect();
    let template= items[0];
    let args: Vec<&str> = items[1].split(" ").collect();
    let mut result = String::new();
    for kind in args {
        result += template.replace("T", kind).as_str();
    }

    result.parse().unwrap()
}

