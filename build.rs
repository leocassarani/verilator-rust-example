use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

const TOP: &str = "top";

fn main() {
    println!("cargo:rerun-if-changed=src/{TOP}.v");
    println!("cargo:rerun-if-changed=src/v{TOP}_bridge.cpp");
    println!("cargo:rerun-if-changed=src/verilated_bridge.cpp");

    let out_dir_env = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_env);

    let verilator_root = Command::new("verilator")
        .args(["--getenv", "VERILATOR_ROOT"])
        .output()
        .map(|output| PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()))
        .unwrap();

    let status = Command::new("verilator")
        .arg("-cc")
        .args(["-Mdir", out_dir.to_str().unwrap()])
        .args(["--top-module", TOP])
        .arg(format!("src/{TOP}.v"))
        .arg(format!("src/sub.v"))
        .status()
        .unwrap();

    if !status.success() {
        std::process::exit(status.code().unwrap())
    }

    let verilated_cpp_files: Vec<_> = std::fs::read_dir(out_dir)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            match path.extension().and_then(OsStr::to_str) {
                Some("cpp") => Some(path),
                _ => None,
            }
        })
        .collect();

    let verilator_include = verilator_root.join("include");

    cc::Build::new()
        .cpp(true)
        .include(&verilator_include)
        .include(out_dir)
        .file(&verilator_include.join("verilated.cpp"))
        .file(&verilator_include.join("verilated_vcd_c.cpp"))
        .files(&verilated_cpp_files)
        .file(format!("src/v{TOP}_bridge.cpp"))
        .file(format!("src/verilated_bridge.cpp"))
        .compile("verilated");
}
