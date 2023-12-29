use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use tracing::{event, Level};
use url::Url;

#[derive(Serialize)]
pub struct ShareResult<'a> {
    status: &'a str,
}
const ERROR_RETURN: Json<ShareResult> = Json(ShareResult { status: "error" });

const SUCCESS_RETURN: Json<ShareResult> = Json(ShareResult { status: "success" });

#[derive(Debug, Deserialize)]
pub struct Params {
    pub url: Option<String>,
}

pub async fn share_url(
    Path(table): Path<String>,
    params: Option<Query<Params>>,
) -> (StatusCode, Json<ShareResult<'static>>) {
    // todo verify table
    let _table = table.to_lowercase();

    let Query(params) = match params {
        Some(p) => p,
        None => {
            event!(Level::INFO, "share without url");
            return (StatusCode::NO_CONTENT, ERROR_RETURN);
        }
    };

    let parsed_url = match Url::parse(params.url.unwrap().as_str()) {
        Ok(url) => url,
        Err(_) => {
            todo!("url error handling not implemented")
        }
    };

    event!(Level::INFO, "sharing url {}", parsed_url.to_string());

    (StatusCode::OK, SUCCESS_RETURN)
}
