// use rauthy_models::i18n_email::ui::I18n;
// use rauthy_models::language::Language;
// use std::fs;
// use std::process::Command;
// use strum::IntoEnumIterator;
//
fn main() {
    //         let dir = "../../frontend/src/i18n_email";
    //         println!("Building I18n JSON for UI DEV SSR in {}", dir);
    //
    //         fs::create_dir_all(dir).unwrap();
    //         fs::write(
    //             format!("{}/DO_NOT_EDIT.txt", dir),
    //             r#"Do NOT edit the files in this directory.
    //     These are automatically managed by the build script in 'src/bin/build.rs' and will be overwritten.
    //             "#,
    //         )
    //         .unwrap();
    //
    //         for lang in Language::iter() {
    //             let i18n_email = I18n::build(&lang);
    //             let path = format!("{}/{}.json", dir, lang.as_str());
    //             let json = serde_json::to_string(&i18n_email).unwrap();
    //             fs::write(path, json).unwrap();
    //         }
    //
    //         Command::new("git")
    //             .arg("add")
    //             .arg(format!("{dir}/*"))
    //             .spawn()
    //             .unwrap();
}
