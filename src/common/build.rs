fn main() {
    println!(
        "cargo::rustc-env=BUILD_TIME={}",
        chrono::Utc::now().timestamp()
    );
}
