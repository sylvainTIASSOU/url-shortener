use axum::http::{HeaderName, Method};
use axum::{middleware, Router};
use axum::routing::{get, post};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::config::config::Config;
use crate::controllers::link;
use crate::middlewares::trace::{init_tracing, require_api_key};

pub struct App;

const TODO_TAG: &str = "todo";
impl App {
    pub async fn run() {
        #[derive(OpenApi)]
        #[openapi(
            // modifiers(&SecurityAddon),
            tags(
            (name = TODO_TAG, description = "Url shortener axum api project"),
            )
        )]
        struct ApiDoc;

        dotenvy::dotenv().ok(); // load .env if present, ignore if missing
        init_tracing();



        let request_id_header = HeaderName::from_static("x-request-id");
        // SetRequestIdLayer generates a UUID and puts it in the request header.
        // PropagateRequestIdLayer copies it to the response header so clients can trace requests.

        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(vec![Method::GET])
            .allow_headers(Any);
        //In production, replace Any with your actual allowed origins:
        //.allow_origin("https://yourdomain.com".parse::<HeaderValue>().unwrap())

        let protected = Router::new()
            .route("/links", post(link::create_link))
            .route_layer(middleware::from_fn(require_api_key));

        let config = Config::from_env().unwrap_or_else(|e| {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        });
        //.post(link::create_link)
        println!("starting on port {}", config.port);
        let app = Router::new()
            .route("/health", get(link::health))
            .route("/links", get(link::list_links))
            .route("/links/{code}", get(link::get_link))
            .layer(TraceLayer::new_for_http())
            .layer(SetRequestIdLayer::new(request_id_header, MakeRequestUuid))
            .layer(cors).merge(protected)
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));;


        let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .unwrap();
        println!("==========================Axum server start ==========================");
        println!(
            "          Server is listening on: http://{}",
            listener.local_addr().unwrap()
        );
        println!("=======================================================================");
        axum::serve(listener, app).await.unwrap();
    }
}