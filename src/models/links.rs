use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub code: String,
    pub url: String,
    pub clicks: u64,
    // #[serde(skip)]
    // internal_id: Option<u64>, // never appears in JSON output
}

#[derive(Deserialize, Serialize)]
pub struct CreateLink {
    pub url: String,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}
//
// #[derive(Serialize, Deserialize)]
// struct UpdateLink {
//     url: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     label: Option<String>, // omitted from JSON if None
// }