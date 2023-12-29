use axum::extract::{Path, Query};
use axum::{handler::HandlerWithoutStateExt, http::StatusCode, Router};
use axum::{routing::get, Json};
use serde::{Deserialize, Serialize};

use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{event, Level};
use url::Url;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // debug is used for all URL requests
        .init();

    /*
    // routes from golang implementation
    router.HandleFunc("/", startpage.Dispatch)
    router.HandleFunc("/k/check/json/{namespace}", check.DispatchJSON)

    // should actually be share/get as we don't do json here
    router.HandleFunc("/k/share/json/{namespace}", share.DispatchJSON)
    router.HandleFunc("/k/share/slack/{namespace}", share.DispatchSlack)

    router.HandleFunc("/k/update/json/{namespace}", update.DispatchJSON)

    router.HandleFunc("/k/show/json/{namespace}", show.DispatchJSON)
    router.HandleFunc("/k/show/www/{namespace}", show.DispatchWWW)
    router.HandleFunc("/k/show/rss/{namespace}", show.DispatchRSS)

    router.HandleFunc("/c/clear_test/", cron.ClearTest)
    router.HandleFunc("/c/clear_test", cron.ClearTest)

    http.HandleFunc("/_ah/mail/", email.DispatchEmail)
     */
    // setup all routes
    let app = Router::new()
        .route("/k/share/:table", get(share_url))
        // serve static html for everything not matched by a route
        .fallback_service(ServeDir::new("static").not_found_service(not_found.into_service()))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// the input to our `create_user` handler
#[derive(Serialize)]
struct Status<'a> {
    status: &'a str,
}
const ERROR_RETURN: Json<Status> = Json(Status { status: "error" });

const SUCCESS_RETURN: Json<Status> = Json(Status { status: "success" });

#[derive(Debug, Deserialize)]
struct Params {
    pub url: Option<String>,
}

async fn share_url(
    Path(table): Path<String>,
    params: Option<Query<Params>>,
) -> (StatusCode, Json<Status<'static>>) {
    // todo verify table
    let _table = table.to_lowercase();

    let Query(params) = match (params) {
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

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 🤷‍♂️")
}
