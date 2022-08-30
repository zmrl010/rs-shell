use std::fs;

/// This module is almost a verbatim copy of uutils - uu_cat
/// Credit goes to these talented developers
/// (c) Jordi Boggiano <j.boggiano@seld.be>
/// (c) Evgeniy Klyuchikov <evgeniy.klyuchikov@gmail.com>
/// (c) Joshua S. Miller <jsmiller@uchicago.edu>
/// (c) Árni Dagur <arni@dagur.eu>

/// Options passed to cat command
struct CatOptions {
    number: bool,
}

/// concatenate files and print on the standard output
/// ```
/// cat [OPTION]... [FILE]...
/// ```
/// # Examples
///
pub fn cat(options: &CatOptions, files: &[String]) -> Result<String, &'static str> {
    if files.len() < 1 {
        return Err("No files were passed");
    }

    let result: Vec<&String> = Vec::new();

    Ok(String::from(""))
}
