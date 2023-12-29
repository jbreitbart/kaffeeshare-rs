use axum::extract::Path;
use axum::{handler::HandlerWithoutStateExt, http::StatusCode, Router};
use axum::{routing::get, routing::post, Json};
use serde::{Deserialize, Serialize};

use tower_http::{services::ServeDir, trace::TraceLayer};

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
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        // serve static html for everything not matched by a route
        .fallback_service(ServeDir::new("static").not_found_service(not_found.into_service()))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn share_url(Path(table): Path<String>) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: table,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 🤷‍♂️")
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
