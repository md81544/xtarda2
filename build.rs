fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos" {
        let brew_prefix = std::process::Command::new("brew")
            .args(["--prefix"])
            .output()
            .expect("failed to run brew")
            .stdout;
        let brew_prefix = String::from_utf8(brew_prefix).unwrap();
        let brew_prefix = brew_prefix.trim();

        println!("cargo::rustc-link-search=native={}/lib", brew_prefix);
        println!("cargo::rustc-link-arg=-ObjC");
    }
}