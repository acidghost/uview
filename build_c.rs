extern crate gcc;

fn main() {
    if cfg!(target_os = "linux") {
        gcc::compile_library("libsysinfo.a", &["src/c/linux.c"]);
    } else if cfg!(target_os = "macos") {
        gcc::compile_library("libsysinfo.a", &["src/c/macos.c"]);
    } else {
        panic!("Unsupported system");
    }
}
