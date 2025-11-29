use pulldown_cmark::{Options, Parser};

pub fn render_sanitized_markdown(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all());
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
