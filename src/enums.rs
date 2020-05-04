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
#[derive(Debug, Clone, Copy)]
pub enum RankingMode {
    Daily,
    Weekly,
    Monthly,
    Rookie,
    Original,
    Male,
    Female,
    DailyR18,
    WeeklyR18,
    MaleR18,
    FemaleR18,
    R18G,
}

impl RankingMode {
    pub fn as_str(&self) -> &'static str {
        match *self {
            RankingMode::Daily => "daily",
            RankingMode::Weekly => "weekly",
            RankingMode::Monthly => "monthly",
            RankingMode::Rookie => "rookie",
            RankingMode::Original => "original",
            RankingMode::Male => "male",
            RankingMode::Female => "female",
            RankingMode::DailyR18 => "daily_r18",
            RankingMode::WeeklyR18 => "weekly_r18",
            RankingMode::MaleR18 => "male_r18",
            RankingMode::FemaleR18 => "female_r18",
            RankingMode::R18G => "r18g",
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
