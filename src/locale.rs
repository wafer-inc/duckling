/// Supported languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    /// English
    EN,
}

impl Lang {
    /// Returns the ISO 639-1 language code.
    pub fn code(&self) -> &'static str {
        match self {
            Lang::EN => "en",
        }
    }
}

/// Supported regions for locale-specific behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Region {
    /// United States
    US,
    /// United Kingdom
    GB,
    /// Australia
    AU,
    /// Canada
    CA,
    /// India
    IN,
}

impl Region {
    /// Returns the ISO 3166-1 alpha-2 region code.
    pub fn code(&self) -> &'static str {
        match self {
            Region::US => "US",
            Region::GB => "GB",
            Region::AU => "AU",
            Region::CA => "CA",
            Region::IN => "IN",
        }
    }
}

/// A locale combining a language and optional region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Locale {
    /// The language.
    pub lang: Lang,
    /// Optional region for locale-specific rules.
    pub region: Option<Region>,
}

impl Locale {
    /// Create a new locale.
    pub fn new(lang: Lang, region: Option<Region>) -> Self {
        Locale { lang, region }
    }
}

impl Default for Locale {
    fn default() -> Self {
        Locale {
            lang: Lang::EN,
            region: Some(Region::US),
        }
    }
}
