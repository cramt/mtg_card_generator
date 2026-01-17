/// Sanitizes a card name for use as a filename.
///
/// Converts to lowercase, replaces spaces and special characters with underscores,
/// and removes any characters that aren't alphanumeric or underscores.
///
/// # Examples
///
/// ```
/// use mtg_gen::sanitize_card_name;
///
/// assert_eq!(sanitize_card_name("Llanowar Elves"), "llanowar_elves");
/// assert_eq!(sanitize_card_name("Fire // Ice"), "fire_ice");
/// assert_eq!(sanitize_card_name("Jace, the Mind Sculptor"), "jace_the_mind_sculptor");
/// assert_eq!(sanitize_card_name("Emeria's Call"), "emerias_call");
/// assert_eq!(sanitize_card_name("Delver of Secrets // Insectile Aberration"), "delver_of_secrets_insectile_aberration");
/// ```
#[must_use]
pub fn sanitize_card_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c)
            } else if c.is_whitespace() || c == '/' || c == ',' || c == '-' {
                Some('_')
            } else if c == '\'' {
                // Remove apostrophes entirely
                None
            } else {
                // Remove other special characters (like æ, ñ, etc.)
                None
            }
        })
        .collect::<String>()
        // Replace multiple consecutive underscores with a single one
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_simple_name() {
        assert_eq!(sanitize_card_name("Llanowar Elves"), "llanowar_elves");
    }

    #[test]
    fn test_sanitize_split_card() {
        assert_eq!(sanitize_card_name("Fire // Ice"), "fire_ice");
    }

    #[test]
    fn test_sanitize_with_comma() {
        assert_eq!(
            sanitize_card_name("Jace, the Mind Sculptor"),
            "jace_the_mind_sculptor"
        );
    }

    #[test]
    fn test_sanitize_with_apostrophe() {
        assert_eq!(sanitize_card_name("Emeria's Call"), "emerias_call");
    }

    #[test]
    fn test_sanitize_dfc() {
        assert_eq!(
            sanitize_card_name("Delver of Secrets // Insectile Aberration"),
            "delver_of_secrets_insectile_aberration"
        );
    }

    #[test]
    fn test_sanitize_with_hyphen() {
        assert_eq!(
            sanitize_card_name("Tok-Tok, Volcano Born"),
            "tok_tok_volcano_born"
        );
    }

    #[test]
    fn test_sanitize_multiple_spaces() {
        assert_eq!(
            sanitize_card_name("Card  With   Spaces"),
            "card_with_spaces"
        );
    }

    #[test]
    fn test_sanitize_special_characters() {
        assert_eq!(sanitize_card_name("Ætherling"), "therling");
    }

    #[test]
    fn test_sanitize_numbers() {
        assert_eq!(
            sanitize_card_name("Phyrexian Fleshgorger"),
            "phyrexian_fleshgorger"
        );
    }
}
