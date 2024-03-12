#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use libc::{c_char, c_int};

mod ffi;

#[derive(Debug, ZvalConvert)]
pub struct SpellResult {
    //misspelled: String,
    //pos: usize,
    line: usize,
    //suggestions: Option<Vec<String>>,
}

#[php_class]
pub struct SpellCheck {
    spellchecker: *mut ffi::AspellSpeller,
}

#[php_impl]
impl SpellCheck {
    // Build an Aspell instance
    pub fn __construct(locale: &str) -> PhpResult<Self> {
        let base_config =  unsafe { 
            ffi::new_aspell_config() 
        };

        if base_config.is_null() {
            return Err(
                PhpException::default("Aspell build config error.".to_string())
            );
        }

        unsafe {
            ffi::aspell_config_replace(base_config, "lang".as_ptr() as *const c_char, locale.as_ptr() as *const c_char)
        };

        let possible_err = unsafe {
            ffi::new_aspell_speller(base_config)
        };       

        if !possible_err.is_null() {
            let spell_checker = unsafe {
                ffi::to_aspell_speller(possible_err)
            };

            if !spell_checker.is_null() {
                return Err(
                    PhpException::default("Aspell speller instance creation error.".to_string())
                );
            }

            Ok(Self {
                spellchecker: spell_checker,
            })
        } else {
            Err(
                PhpException::default("Aspell speller instance creation error.".to_string())
            )
        }
    }

    // Check misspelled word
    pub fn check(&mut self, name: &str) -> Vec<SpellResult> {
        let lines = name.lines();
        let mut results: Vec<SpellResult> = Vec::new();

        for (line_number, line_content) in lines.enumerate() {

            let errors = unsafe {
                ffi::aspell_speller_check(self.spellchecker, line_content.as_ptr() as *const c_char, line_content.len() as c_int)
            };
            
            results.push(SpellResult {
                //misspelled: e.misspelled,
                //pos: e.position,
                line: line_number,
                // suggestions: Some(e.suggestions)
            });
        }

        results
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
