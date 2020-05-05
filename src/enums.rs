use serde::{Deserialize, Serialize};

// TODO: Specificy how serde should deserialize these...
#[derive(Deserialize, Serialize, Debug)]
pub enum SearchTarget {
    TagsPartial,
    TagsExact,
    TitleAndCaption,
}

impl SearchTarget {
    pub fn as_str(&self) -> &'static str {
        match *self {
            SearchTarget::TagsPartial => "partial_match_for_tags",
            SearchTarget::TagsExact => "exact_match_for_tags",
            SearchTarget::TitleAndCaption => "title_and_caption",
        }
    }

    pub fn map<U, F: FnOnce(Self) -> U>(self, f: F) -> U {
        f(self)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ContentType {
    #[serde(rename = "illust")]
    Illustration,
    #[serde(rename = "manga")]
    Manga,
    #[serde(rename = "ugoira")]
    Ugoira,
    #[serde(rename = "novel")]
    Novel,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ContentType::Illustration => "illust",
            ContentType::Manga => "manga",
            ContentType::Ugoira => "ugoira",
            ContentType::Novel => "novel",
        }
    }
}

/// Enum to set publicity param.
#[derive(Debug, Clone, Copy)]
pub enum Publicity {
    Public,
    Private,
}

impl Publicity {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Publicity::Public => "public",
            Publicity::Private => "private",
        }
    }
}

/// Enum to set ranking type param.
#[derive(Debug, Clone, Copy)]
pub enum RankingType {
    All,
    Illust,
    Manga,
    Ugoira,
}

impl RankingType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            RankingType::All => "all",
            RankingType::Illust => "illust",
            RankingType::Manga => "manga",
            RankingType::Ugoira => "ugoira",
        }
    }
}

/// Enum to set ranking mode param.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RankingMode {
    #[serde(rename = "day")]
    Daily,
    #[serde(rename = "week")]
    Weekly,
    #[serde(rename = "month")]
    Monthly,
    #[serde(rename = "day_male")]
    DayMale,
    #[serde(rename = "day_female")]
    DayFemale,
    #[serde(rename = "week_original")]
    WeekOriginal,
    #[serde(rename = "week_rookie")]
    WeekRookie,
    #[serde(rename = "day_manga")]
    DayManga,
}

impl RankingMode {
    pub fn as_str(&self) -> &'static str {
        match *self {
            RankingMode::Daily => "day",
            RankingMode::Weekly => "week",
            RankingMode::Monthly => "month",
            RankingMode::DayMale => "day_male",
            RankingMode::DayFemale => "day_female",
            RankingMode::WeekOriginal => "week_original",
            RankingMode::WeekRookie => "week_rookie",
            RankingMode::DayManga => "day_manga",
        }
    }
}

/// Enum to set search period param.
#[derive(Debug, Clone, Copy)]
pub enum SearchPeriod {
    All,
    Day,
    Week,
    Month,
}

impl SearchPeriod {
    pub fn as_str(&self) -> &'static str {
        match *self {
            SearchPeriod::All => "all",
            SearchPeriod::Day => "day",
            SearchPeriod::Week => "week",
            SearchPeriod::Month => "month",
        }
    }
}

/// Enum to set search mode param.
#[derive(Debug, Clone, Copy)]
pub enum SearchMode {
    Text,
    Tag,
    ExactTag,
    Caption,
}

impl SearchMode {
    pub fn as_str(&self) -> &'static str {
        match *self {
            SearchMode::Text => "text",
            SearchMode::Tag => "tag",
            SearchMode::ExactTag => "exact_tag",
            SearchMode::Caption => "caption",
        }
    }
}

/// Enum to set search order param.
#[derive(Debug, Clone, Copy)]
pub enum SearchOrder {
    Descending,
    Ascending,
}

impl SearchOrder {
    pub fn as_str(&self) -> &'static str {
        match *self {
            SearchOrder::Descending => "desc",
            SearchOrder::Ascending => "asc",
        }
    }
}

/// Enum to sort search result.
#[derive(Deserialize, Serialize, Debug)]
pub enum SearchSort {
    DateAscending,
    DateDescending,
}

impl SearchSort {
    pub fn as_str(&self) -> &'static str {
        match *self {
            SearchSort::DateAscending => "date_asc",
            SearchSort::DateDescending => "date_desc",
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Duration {
    LastDay,
    LastWeek,
    LastMonth,
}

impl Duration {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Duration::LastDay => "within_last_day",
            Duration::LastWeek => "within_last_week",
            Duration::LastMonth => "within_last_month",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl Visibility {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Visibility::Public => "public",
            Visibility::Private => "private",
        }
    }
}

impl From<bool> for Visibility {
    fn from(x: bool) -> Self {
        match x {
            true => Visibility::Public,
            false => Visibility::Private,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Filter {
    ForiOS,
}

impl Filter {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Filter::ForiOS => "for_ios",
        }
    }
}
