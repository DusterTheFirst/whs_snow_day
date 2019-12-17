use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PrePosts {
    pub posts: Vec<Post>,
    pub hash: u64
}

#[derive(Debug, Deserialize, Serialize, Hash)]
pub struct Post {
    pub id: u32,
    pub date: String,
    pub date_gmt: String,
    pub modified: String,
    pub modified_gmt: String,
    pub slug: String,
    pub link: String,
    #[serde(with = "nested_rendered")]
    pub title: String,
    #[serde(with = "nested_rendered")]
    pub content: String,
    #[serde(with = "nested_rendered")]
    pub excerpt: String,
}

mod nested_rendered {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Debug, Deserialize, Serialize)]
    struct Rendered {
        rendered: String,
    }

    pub fn deserialize<'j, D: Deserializer<'j>>(deserializer: D) -> Result<String, D::Error> {
        Rendered::deserialize(deserializer).map(|a| a.rendered)
    }
    pub fn serialize<S: Serializer>(value: &String, serializer: S) -> Result<S::Ok, S::Error> {
        Rendered::serialize(
            &Rendered {
                rendered: value.to_string(),
            },
            serializer,
        )
    }
}
