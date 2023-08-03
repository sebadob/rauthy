use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct I18nPasswordPolicy<'a> {
    password_policy: &'a str,
    length_min: &'a str,
    length_max: &'a str,
    lowercase_min: &'a str,
    uppercase_min: &'a str,
    digits_min: &'a str,
    special_min: &'a str,
    not_recent: &'a str,
}

impl I18nPasswordPolicy<'_> {
    pub fn build_en() -> Self {
        Self {
            password_policy: "Password Policy",
            length_min: "Length min",
            length_max: "Length max",
            lowercase_min: "Lowercase letters min",
            uppercase_min: "Uppercase letters min",
            digits_min: "Digits min",
            special_min: "Special characters min",
            not_recent: "Not one of last recent passwords",
        }
    }

    pub fn build_de() -> Self {
        Self {
            password_policy: "Passwort Regeln",
            length_min: "Länge min",
            length_max: "Länge max",
            lowercase_min: "Kleinbuchstaben min",
            uppercase_min: "Großbuchstaben min",
            digits_min: "Ziffern min",
            special_min: "Spezielle Buchstaben min",
            not_recent: "Keins der letzten Passwörter",
        }
    }
}
