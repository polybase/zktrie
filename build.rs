use std::env;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    if let Some(target) = env::var("TARGET").ok() {
        if target.contains("android") || target.contains("ios") {
            println!("cargo:warning=Building for Android or iOS");
            return;
        }
    } else {
        println!("cargo:warning=Building for non-mobile platform");
    }

   return
}

fn fail(message: String) {
    let _ = writeln!(
        io::stderr(),
        "\n\nError while building zktrie: {message}\n\n",
    );
    std::process::exit(1);
}
