use std::path::PathBuf;

pub struct Config {
    fatal: bool,
    max_depth: u8,
    no_glob: bool,
    silent: bool,
    verbose: bool,
    exec_path: PathBuf,
    buf_len: u32,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let exec_path = match args.next() {
            Some(arg) => PathBuf::from(arg),
            None => return Err("Didn't get exec path"),
        };

        Ok(Config {
            fatal: false,
            max_depth: 255,
            no_glob: false,
            silent: false,
            verbose: false,
            exec_path,
            buf_len: 65536, // 64 KB
        })
    }
}
