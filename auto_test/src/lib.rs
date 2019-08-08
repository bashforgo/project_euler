extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};

fn concat(mut acc: TokenStream2, cur: TokenStream2) -> TokenStream2 {
    acc.extend(cur);
    acc
}

#[proc_macro]
pub fn auto_test(_input: TokenStream) -> TokenStream {
    let mods: Vec<_> = read_dir("src/problems")
        .unwrap()
        .map(|result| result.unwrap())
        .map(|path| path.file_name().into_string().unwrap())
        .map(|file_name| file_name.replace(".rs", ""))
        .filter(|mod_name| mod_name.starts_with('p'))
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

    let solvers = mods
        .iter()
        .map(|module| {
            let module_ident = Ident::new(module, Span::call_site());
            let number = String::from(&module[1..])
                .parse::<u32>()
                .unwrap()
                .to_string();
            quote! {
                solvers.insert(String::from(#number), Box::new(#module_ident::solve));
            }
        })
        .fold(TokenStream2::new(), concat);
    let solvers = quote! {
        pub fn solvers() -> std::collections::HashMap<String, Box<dyn Fn() -> String>> {
            use std::collections::HashMap;
            let mut solvers: HashMap<String, Box<dyn Fn() -> String>> = HashMap::new();

            #solvers

            solvers
        }
    };

    let mut result = mods
        .iter()
        .map(|module| {
            let module_ident = Ident::new(module, Span::call_site());
            let test_module = Ident::new(&format!("test_{}", module), module_ident.span());
            let unit_test = Ident::new(&format!("{}_solution", module), module_ident.span());
            let answer = solutions.get(module);
            quote! {
                mod #module_ident;

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
        .fold(TokenStream2::new(), concat);

    result.extend(solvers);

    result.into()
}
