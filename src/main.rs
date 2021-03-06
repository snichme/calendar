mod cal;
use askama::Template;
use axum::{
    body::{self, Full},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use cal::CalView;
use chrono::Datelike;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_templates=debug")
    }

    // build our application with some routes
    let app = Router::new().route("/", get(index));

    let port = env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .expect("PORT!");
    // run it

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct CalendarTemplate {
    cal: CalView,
}

async fn index() -> impl IntoResponse {
    let now = chrono::offset::Local::today();
    let cal = CalView::new(now);
    HtmlTemplate(CalendarTemplate { cal })
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body::boxed(Full::from(format!(
                    "Failed to render template. Error: {}",
                    err
                ))))
                .unwrap(),
        }
    }
}
