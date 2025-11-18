use pulldown_cmark::{Options, Parser};
use wasm_bindgen::prelude::wasm_bindgen;

/// Renders the given markdown input to a sanitized HTML.
#[wasm_bindgen]
pub fn render_markdown(markdown: String) -> String {
    let parser = Parser::new_ext(&markdown, Options::all());
    // this capacity is not exactly right, but it is a lot closer than just `String::new()`
    let mut html_output = String::with_capacity(markdown.len());
    pulldown_cmark::html::push_html(&mut html_output, parser);

    ammonia::Builder::default()
        .add_generic_attributes(&["align", "cellpadding", "cellspacing", "id"])
        .add_tags(&["input"])
        .add_tag_attributes("input", &["disabled", "checked"])
        .add_tag_attribute_values("input", "type", &["checkbox"])
        .set_tag_attribute_value("a", "target", "_blank")
        .clean(&html_output)
        .to_string()
        .trim()
        .to_string()
}

#[wasm_bindgen]
pub fn sanitize_html(input: String) -> String {
    ammonia::Builder::default()
        .add_generic_attributes(&["align", "cellpadding", "cellspacing", "id", "style"])
        .add_tags(&["input", "path", "svg"])
        .add_tag_attributes("input", &["disabled", "checked"])
        .add_tag_attributes("path", &["d", "stroke-linecap", "stroke-linejoin"])
        .add_tag_attributes(
            "svg",
            &[
                "fill",
                "height",
                "opacity",
                "stroke",
                "stroke-width",
                "viewBox",
                "width",
            ],
        )
        .add_tag_attribute_values("input", "type", &["checkbox"])
        .set_tag_attribute_value("a", "target", "_blank")
        .clean(&input)
        .to_string()
        .trim()
        .to_string()
}

// /// CAUTION: Should only be used if the content is displayed inside a sandboxed `iframe`!
// /// Might overwrite global CSS otherwise.
// #[wasm_bindgen]
// pub fn sanitize_html_keep_style(input: String) -> String {
//     ammonia::Builder::default()
//         .add_generic_attributes(&[
//             "align",
//             "border",
//             "cellpadding",
//             "cellspacing",
//             "class",
//             "height",
//             "id",
//             "role",
//             "style",
//             "title",
//             "width",
//         ])
//         .add_generic_attribute_prefixes(&["aria-"])
//         .add_tags(&["input", "path", "style", "svg"])
//         .add_tag_attributes("input", &["checked"])
//         .add_tag_attributes("path", &["d", "stroke-linecap", "stroke-linejoin"])
//         .add_tag_attributes(
//             "svg",
//             &[
//                 "fill",
//                 "height",
//                 "opacity",
//                 "stroke",
//                 "stroke-width",
//                 "viewBox",
//                 "width",
//             ],
//         )
//         .add_tag_attribute_values("input", "type", &["checkbox"])
//         .rm_clean_content_tags(&["style"])
//         .set_tag_attribute_value("a", "target", "_blank")
//         .clean(&input)
//         .to_string()
//         .trim()
//         .to_string()
// }
