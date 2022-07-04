use std::process::Command;
use std::fs;

fn main() {
    Command::new("npm")
        .arg("ci")
        .output()
        .expect("failed to execute process");

    Command::new("npm")
        .arg("run")
        .arg("build")
        .output()
        .expect("failed to execute process");

    fs::copy("node_modules/reveal.js/dist/reveal.js", "src/html/res/reveal.out.js")
        .expect("failed to copy reveal js");

    fs::copy("node_modules/reveal.js/dist/reveal.css", "src/html/res/reveal.out.css")
        .expect("failed to copy reveal css");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=package-lock.json");
}
