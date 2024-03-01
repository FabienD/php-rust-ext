#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use ispell::SpellLauncher;

#[php_function]
pub fn ispell_check(name: &str, locale: &str) -> () {
    let mut checker = SpellLauncher::new()
                 .aspell()
                 .dictionary(locale)
                 .launch()
                 .unwrap();
    
    let errors = checker.check(name).unwrap();
    
    for e in errors {
        println!("{} was misspelled at pos {}.", e.misspelled, e.position);
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
