use crate::enums::Filter;
use crate::enums::RankingMode;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IllustrationRankingRequestArg {
    mode: RankingMode,
    // TODO: Figure out how to encapsulate the date properly.
    date: Option<String>,
    offset: u32,
    filter: Filter,
}

impl Default for IllustrationRankingRequestArg {
    fn default() -> Self {
        IllustrationRankingRequestArg {
            mode: RankingMode::Daily,
            date: None,
            offset: 0,
            filter: Filter::ForiOS,
        }
    }
}

impl IllustrationRankingRequestArg {
    pub fn set_mode<T>(mut self, value: T) -> Self
    where
        T: Into<RankingMode>,
    {
        self.mode = value.into();
        self
    }

    pub fn set_date<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.date = Some(value.into());
        self
    }

    pub fn set_offset(mut self, value: u32) -> Self {
        self.offset = value;
        self
    }

    pub fn set_filter<T>(mut self, value: T) -> Self
    where
        T: Into<Filter>,
    {
        self.filter = value.into();
        self
    }

    pub fn build(self) -> std::collections::HashMap<&'static str, String> {
        let mut result = std::collections::HashMap::new();

        result.insert("mode", self.mode.as_str().to_string());

        if let Some(date) = self.date {
            result.insert("date", date);
        }

        result.insert("offset", self.offset.to_string());
        result.insert("filter", self.filter.as_str().to_string());

        result
    }
}
