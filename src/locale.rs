#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    EN,
}

impl Lang {
    pub fn code(&self) -> &'static str {
        match self {
            Lang::EN => "en",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Region {
    US,
    GB,
    AU,
    CA,
    IN,
}

impl Region {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Locale {
    pub lang: Lang,
    pub region: Option<Region>,
}

impl Locale {
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
