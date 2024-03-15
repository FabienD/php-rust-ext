use libc::{c_int, c_char};

#[cfg(target_os = "linux")]
mod platform {
    #[cfg(target_arch = "x86_64")]
    #[link(name = "aspell")] extern{}
}

#[repr(C)] pub struct AspellConfig { private: [u8; 0] }
#[repr(C)] pub struct AspellSpeller { private: [u8; 0] }
#[repr(C)] pub struct AspellCanHaveError { private: [u8; 0] }
// #[repr(C)] pub struct AspellWordList { private: [u8; 0] }
// #[repr(C)] pub struct AspellStringEnumeration { private: [u8; 0] }

extern "C" {
    pub fn new_aspell_config() -> *mut AspellConfig;
    pub fn aspell_config_replace(config: *mut AspellConfig, key: *const c_char, value: *const c_char);
    // pub fn aspell_config_retrieve(config: *mut AspellConfig, key: *const c_char);
    pub fn delete_aspell_config(config: *mut AspellConfig);
    pub fn new_aspell_speller(config: *mut AspellConfig) -> *mut AspellCanHaveError;
    pub fn aspell_error_number(possible_err: *mut AspellCanHaveError) -> c_int;
    pub fn to_aspell_speller(possible_err: *mut AspellCanHaveError) -> *mut AspellSpeller;
    pub fn aspell_speller_check(speller: *mut AspellSpeller, word: *const c_char, word_size: c_int) -> c_int;
    // pub fn aspell_speller_suggest(speller: *mut AspellSpeller, word: *const c_char, size: c_int) -> *mut AspellWordList;
    // pub fn aspell_word_list_elements(suggestions: *mut AspellWordList) -> *mut AspellStringEnumeration;
    // pub fn delete_aspell_string_enumeration(elements: *mut AspellStringEnumeration);
}
