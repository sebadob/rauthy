use std::fmt::Write;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=../i18n/src/languages/");

    let _ = fs::create_dir("../i18n/i18n_hashes");

    let mut content = String::with_capacity(128);
    for (lang, i18n) in rauthy_i18n::build_all_admin() {
        let hash = i18n.content_hash();
        writeln!(content, "{}_{}", lang, hash).unwrap();
    }
    fs::write("../i18n/i18n_hashes/admin", &content).unwrap();

    content.clear();
    for (lang, i18n) in rauthy_i18n::build_all_common() {
        let hash = i18n.content_hash();
        writeln!(content, "{}_{}", lang, hash).unwrap();
    }
    fs::write("../i18n/i18n_hashes/common", &content).unwrap();
}
