use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ImageUrl {
    pub large: Option<String>,
    pub medium: Option<String>,
    pub small: Option<String>,
    pub square_medium: Option<String>,
}

impl IntoIterator for ImageUrl {
    type Item = String;
    type IntoIter = ImageUrlIterator;

    fn into_iter(self) -> Self::IntoIter {
        ImageUrlIterator {
            url: self,
            index: 0,
        }
    }
}

pub struct ImageUrlIterator {
    url: ImageUrl,
    index: usize,
}

/// TODO: Rework this hack?
impl Iterator for ImageUrlIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let result = match self.index {
                0 => self.url.small.take(),
                1 => self.url.medium.take(),
                2 => self.url.large.take(),
                3 => self.url.square_medium.take(),
                _ => return None,
            };
            self.index += 1;

            if let Some(r) = result {
                return Some(r);
            }
        }
    }
}
