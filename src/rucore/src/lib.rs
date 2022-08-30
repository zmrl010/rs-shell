pub mod config;

#[macro_export]
macro_rules! bin {
    ($util:ident) => {
        pub fn main() {
            use std::io::Write;
            let code = $util::rumain;
            std::io::stdout().flush().expect("could not flush stdout");
            std::process::exit(code);
        }
    };
}
