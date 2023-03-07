use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CursorPagination {
    #[serde_as(as = "DisplayFromStr")]
    #[schemars(with = "i32", default = "default_take", length(min = 1, max = 100))]
    #[serde(default = "default_take")]
    /// The number of items to return. Default is 100.
    pub take: i64,
    /// The pagination cursor to start at.
    pub after: Option<String>,
    /// The pagination cursor to end at.
    pub before: Option<String>,
}

fn default_take() -> i64 {
    100
}
