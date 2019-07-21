extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};

#[proc_macro]
pub fn auto_test(_input: TokenStream) -> TokenStream {
    let mods: Vec<_> = read_dir("src/problems")
        .unwrap()
        .map(|result| result.unwrap())
        .map(|path| path.file_name().into_string().unwrap())
        .map(|file_name| file_name.replace(".rs", ""))
        .filter(|mod_name| mod_name.starts_with("p"))
        .collect();

    let solutions = read_to_string("projecteuler-solutions/Solutions.md").unwrap();
    let solutions: HashMap<_, _> = solutions
        .lines()
        .map(|line| line.split(". ").collect())
        .filter(|split: &Vec<_>| split.len() == 2)
        .map(|solution| {
            if let [index, answer] = solution[..] {
                let index = format!("p{}{}", "0".repeat(3 - index.len()), index);
                (index, String::from(answer))
            } else {
                unreachable!()
            }
        })
        .collect();

    mods.iter()
        .map(|module| {
            let module_ident = Ident::new(module, Span::call_site());
            let mark_as_used =
                Ident::new(&format!("_mark_as_used_{}", module), module_ident.span());
            let test_module = Ident::new(&format!("test_{}", module), module_ident.span());
            let unit_test = Ident::new(&format!("{}_solution", module), module_ident.span());
            let answer = solutions.get(module);
            quote! {
                mod #module_ident;

                #[allow(dead_code)]
                fn #mark_as_used() {
                    #module_ident::solve();
                }

                #[cfg(test)]
                mod #test_module {
                    use super::#module_ident::solve;

                    #[test]
                    fn #unit_test() {
                        let answer = #answer;
                        assert!(answer == solve());
                    }
                }
            }
        })
        .fold(TokenStream2::new(), |acc, cur| {
            quote! {
                #acc

                #cur
            }
        })
        .into()
}
