use crate::enums::{Duration, SearchSort, SearchTarget};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IllustrationSearchParam {
    word: String,
    search_target: SearchTarget,
    sort: SearchSort,
    duration: Option<Duration>,
    offset: Option<i32>,
    filter: Option<&'static str>,
}

impl IllustrationSearchParam {
    pub fn default<T>(word: T) -> Self
    where
        T: Into<String>,
    {
        IllustrationSearchParam {
            word: word.into(),
            search_target: SearchTarget::TagsPartial,
            sort: SearchSort::DateDescending,
            duration: None,
            offset: None,
            filter: None,
        }
    }

    pub fn set_search_target<T>(mut self, search_target: T) -> Self
    where
        T: Into<SearchTarget>,
    {
        self.search_target = search_target.into();
        self
    }

    pub fn set_sort<T>(mut self, sort: T) -> Self
    where
        T: Into<SearchSort>,
    {
        self.sort = sort.into();
        self
    }

    pub fn set_duration<T>(mut self, duration: T) -> Self
    where
        T: Into<Duration>,
    {
        self.duration = Some(duration.into());
        self
    }

    pub fn set_offset<T>(mut self, offset: T) -> Self
    where
        T: Into<i32>,
    {
        self.offset = Some(offset.into());
        self
    }
}

impl IntoIterator for IllustrationSearchParam {
    type Item = (&'static str, String);
    type IntoIter = IllustrationSearchParamIterator;

    fn into_iter(self) -> Self::IntoIter {
        IllustrationSearchParamIterator {
            vec: self,
            index: 0,
        }
    }
}

pub struct IllustrationSearchParamIterator {
    vec: IllustrationSearchParam,
    index: usize,
}

// TODO: Figure out the lifetime of this Iterator...
impl Iterator for IllustrationSearchParamIterator {
    type Item = (&'static str, String);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let result = match self.index {
                0 => Some(("word", self.vec.word.clone())),
                1 => Some(("search_target", self.vec.search_target.as_str().to_string())),
                2 => Some(("sort", self.vec.sort.as_str().to_string())),
                3 => self
                    .vec
                    .duration
                    .take()
                    .map(|x| ("duration", x.as_str().to_string())),
                4 => self.vec.offset.take().map(|x| ("offset", x.to_string())),
                5 => self.vec.filter.take().map(|x| ("filter", x.to_string())),
                _ => return None,
            };
            self.index += 1;
            if let Some(r) = result {
                return Some(r);
            }
        }
    }
}

impl From<&'static str> for IllustrationSearchParam {
    fn from(x: &'static str) -> Self {
        IllustrationSearchParam::default(x)
    }
}
