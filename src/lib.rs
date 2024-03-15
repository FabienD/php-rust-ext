#![cfg_attr(windows, feature(abi_vectorcall))]
use std::ffi::CString;

use ext_php_rs::prelude::*;
use libc::c_char;
use unicode_segmentation::UnicodeSegmentation;

mod ffi;

#[derive(Debug, ZvalConvert)]
pub struct SpellResult {
    misspelled: String,
    pos: usize,
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

        let config_key = CString::new("lang").expect("CString Config key creation failed");
        let config_value = CString::new(locale).expect("CString Config value creation failed");

        unsafe {
            ffi::aspell_config_replace(base_config, config_key.as_ptr() as *const c_char, config_value.as_ptr() as *const c_char)
        };

        let possible_err = unsafe {
            ffi::new_aspell_speller(base_config)
        };

        unsafe {
            ffi::delete_aspell_config(base_config)
        }

        let error_number = unsafe {
            ffi::aspell_error_number(possible_err)
        };

        if error_number == 0 {
            let spell_checker = unsafe {
                ffi::to_aspell_speller(possible_err)
            };

            if spell_checker.is_null() {
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

    // Check misspelled words
    pub fn check(&mut self, text: &str) -> Vec<SpellResult> {
        let mut results = Vec::new();

        // Split by line.
        let lines = text.lines();
        
        for (i, line) in lines.enumerate() {
            // Line word segmentation.
            let seg_words = line.unicode_word_indices().collect::<Vec<(usize, &str)>>();
            
            for word in seg_words {
                let word_to_check = CString::new(word.1).expect("Word creation failed");

                let check = unsafe {
                    ffi::aspell_speller_check(self.spellchecker, word_to_check.as_ptr() as *const c_char, -1)
                };

                if check != 1 {
                    results.push(
                        SpellResult {
                            misspelled: word.1.to_string(),
                            pos: word.0,
                            line: i,
                        }
                    );
                }
            }
        }
        
        results
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
