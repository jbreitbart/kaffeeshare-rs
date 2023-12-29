use axum::extract::Path;
use axum::routing::get;
use axum::Json;
use axum::{handler::HandlerWithoutStateExt, http::StatusCode, Router};

use tower_http::{services::ServeDir, trace::TraceLayer};

pub mod share;

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 🤷‍♂️")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // debug is used for all URL requests
        .init();

    // setup all routes
    let app = Router::new()
        .route("/k/show/json/:table", get(show_json))
        .route("/k/show/www/:table", get(show_www))
        .route("/k/show/rss/:table", get(show_rss))
        .route("/k/share/get/:table", get(share::share_url))
        // used by old extensions
        .route("/k/share/json/:table", get(share::share_url))
        // serve static html for everything not matched by a route
        .fallback_service(ServeDir::new("static").not_found_service(not_found.into_service()))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// temporary handlers

#[derive(serde::Serialize)]
struct TempRes {
    status: String,
}

async fn show_rss(Path(_table): Path<String>) -> (StatusCode, Json<TempRes>) {
    todo!();
}

async fn show_www(Path(_table): Path<String>) -> (StatusCode, Json<TempRes>) {
    todo!();
}

async fn show_json(Path(_table): Path<String>) -> (StatusCode, Json<TempRes>) {
    todo!();
}

/*
// routes from golang implementation
router.HandleFunc("/", startpage.Dispatch)
router.HandleFunc("/k/check/json/{namespace}", check.DispatchJSON)

// should actually be share/get as we don't do json here
router.HandleFunc("/k/share/slack/{namespace}", share.DispatchSlack)

router.HandleFunc("/k/update/json/{namespace}", update.DispatchJSON)

router.HandleFunc("/c/clear_test/", cron.ClearTest)
router.HandleFunc("/c/clear_test", cron.ClearTest)
 */
