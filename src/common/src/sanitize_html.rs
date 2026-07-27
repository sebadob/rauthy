pub fn sanitize_html(input: &str) -> String {
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
        .clean(input)
        .to_string()
        .trim()
        .to_string()
}
