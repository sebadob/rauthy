use std::fs;
use std::io::Write as _;

fn main() {
    // TODO engage again once this is working as expected
    // println!("cargo:rerun-if-changed=../i18n/src/languages/");

    let _ = fs::create_dir("../i18n/i18n_hashes");

    let mut map = phf_codegen::Map::new();
    for (lang, i18n) in rauthy_i18n::build_all_admin() {
        let hash = i18n.content_hash();
        map.entry(lang.as_str(), format!("{hash:?}"));
    }
    let mut file = fs::File::create("../i18n/i18n_hashes/admin").unwrap();
    write!(
        &mut file,
        "static HASHES: ::phf::Map<&'static str, &'static str> = {};",
        map.build()
    )
    .unwrap();

    let mut map = phf_codegen::Map::new();
    for (lang, i18n) in rauthy_i18n::build_all_common() {
        let hash = i18n.content_hash();
        map.entry(lang.as_str(), format!("{hash:?}"));
    }
    let mut file = fs::File::create("../i18n/i18n_hashes/common").unwrap();
    write!(
        &mut file,
        "static HASHES: ::phf::Map<&'static str, &'static str> = {};",
        map.build()
    )
    .unwrap();
}
