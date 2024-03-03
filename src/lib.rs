#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use ispell::{SpellChecker, SpellLauncher};

#[derive(Debug, ZvalConvert)]
pub struct SpellResult {
    misspelled: String,
    pos: usize,
    line: usize,
    suggestions: Option<Vec<String>>,
}

#[php_class]
pub struct SpellCheck {
    checker: SpellChecker,
}

#[php_impl]
impl SpellCheck {
    // Build an Aspell instance
    pub fn __construct(locale: String) -> Self {
        let checker = SpellLauncher::new()
            .aspell()
            .dictionary(locale)
            .launch()
            .unwrap();

        Self { checker }
    }

    // Check misspelled word
    pub fn check(&mut self, name: &str) -> Vec<SpellResult> {
        let lines = name.lines();
        let mut results: Vec<SpellResult> = Vec::new();

        for (line_number, line_content) in lines.enumerate() {
            let errors = self.checker.check(line_content).unwrap();
            for e in errors {
                results.push(SpellResult {
                        misspelled: e.misspelled,
                        pos: e.position,
                        line: line_number,
                        suggestions: Some(e.suggestions)
                    }
                );
            }
        }

        results
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
