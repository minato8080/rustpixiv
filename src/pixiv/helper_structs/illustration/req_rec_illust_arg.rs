use crate::enums::ContentType;

use serde::{Deserialize, Serialize};

/// IllustRecArg (Request Recommended Illustration Arguments Builder).
/// Builds a `IllustRecArg`
#[derive(Debug, Serialize, Deserialize)]
pub struct IllustRecArg {
    content_type: ContentType,
    include_ranking_illustrations: bool,
    max_bookmark_id_for_recommend: Option<u32>,
    min_bookmark_id_for_recent_illustrations: Option<u32>,
    offset: u32,
    bookmark_illust_ids: Vec<u32>,
    include_ranking_label: bool,
}

impl Default for IllustRecArg {
    fn default() -> Self {
        IllustRecArg {
            content_type: ContentType::Illustration,
            include_ranking_illustrations: false,
            max_bookmark_id_for_recommend: None,
            min_bookmark_id_for_recent_illustrations: None,
            offset: 0,
            bookmark_illust_ids: Vec::new(),
            include_ranking_label: false,
        }
    }
}

impl IllustRecArg {
    pub fn content_type<T>(mut self, value: T) -> Self
    where
        T: Into<ContentType>,
    {
        self.content_type = value.into();
        self
    }

    pub fn include_ranking_illustrations<T>(mut self, value: T) -> Self
    where
        T: Into<bool>,
    {
        self.include_ranking_illustrations = value.into();
        self
    }

    pub fn set_max_bookmark_id(mut self, value: u32) -> Self {
        self.max_bookmark_id_for_recommend = Some(value);
        self
    }

    pub fn min_bookmark_id_for_recent_illustrations(mut self, value: u32) -> Self {
        self.min_bookmark_id_for_recent_illustrations = Some(value);
        self
    }

    pub fn offset(mut self, value: u32) -> Self {
        self.offset = value;
        self
    }

    /// Consume a Vec of bookmark ids
    pub fn bookmark_illust_ids<'a, T>(mut self, other: T) -> Self
    where
        T: Into<Vec<u32>>,
    {
        self.bookmark_illust_ids = other.into();
        self
    }

    pub fn include_ranking_label<T>(mut self, value: T) -> Self
    where
        T: Into<bool>,
    {
        self.include_ranking_label = value.into();
        self
    }

    pub fn build(self) -> std::collections::HashMap<&'static str, String> {
        let mut params = std::collections::HashMap::new();

        params.insert("content_type", self.content_type.as_str().into());
        params.insert(
            "include_ranking_label",
            self.include_ranking_label.to_string(),
        );

        if let Some(value) = self.max_bookmark_id_for_recommend {
            params.insert("max_bookmark_id_for_recommend", value.to_string());
        }

        if let Some(value) = self.min_bookmark_id_for_recent_illustrations {
            params.insert("min_bookmark_id_for_recent_illust", value.to_string());
        }

        params.insert("offset", self.offset.to_string());
        params.insert(
            "include_ranking_illusts",
            self.include_ranking_illustrations.to_string(),
        );

        let bookmark_illust_ids = self
            .bookmark_illust_ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        params.insert("bookmark_illust_ids", bookmark_illust_ids);

        params
    }
}
