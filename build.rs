extern crate syntex;
extern crate serde_codegen;

use std::env;
use std::path::Path;

fn main() {
    emit_files("src/types.rs.in", "types.rs");
    emit_files("src/config.rs.in", "config.rs");
}

fn emit_files(src: &str, dst: &str) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let src = Path::new(src);
    let dst = Path::new(&out_dir).join(dst);

    let mut registry = syntex::Registry::new();

    serde_codegen::register(&mut registry);
    registry.expand("", &src, &dst).unwrap();
}
