fn main() {
    // TODO engage again once this is working as expected
    // println!("cargo:rerun-if-changed=../i18n/src/languages/");

    rauthy_i18n_prep::execute().unwrap();
}
