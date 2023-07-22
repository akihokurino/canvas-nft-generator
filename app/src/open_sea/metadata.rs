use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String,
}

impl Metadata {
    pub fn new(name: &str, description: &str, image_url: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            image: image_url.to_string(),
        }
    }
}
