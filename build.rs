use std::env;
use std::ffi;
use std::fs;
use std::io::Write;
use std::path;

fn main() {
    let scripts_rs = path::Path::new(&env::var("OUT_DIR").unwrap()).join("scripts.rs");
    let mut rs = fs::File::create(&scripts_rs).unwrap();

    for script in fs::read_dir("tests").unwrap() {
        let script = script.unwrap();
        let path = script.path();
        if path.extension() != Some(ffi::OsStr::new("lam")) {
            continue;
        }
        let path = path.canonicalize().unwrap();
        let fullpath = path.to_str().unwrap();
        let stem = path.file_stem().unwrap().to_str().unwrap();
        write!(
            rs,
            "#[test]\nfn test_script_{}() {{ test_script(r\"{}\"); }}\n",
            stem, fullpath
        ).unwrap();
    }
}
