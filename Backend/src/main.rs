use axum::{
    Router,
    routing::post,
    Json,
    http::{HeaderValue, Method, header}
};
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders, AllowMethods};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use chrono::Datelike;

mod zodiac_list;
mod rashi_date_range;

use zodiac_list::ZodiacList;
use rashi_date_range::{find_zodiac_sign, get_lucky_colors};

#[tokio::main]
async fn main() {
    // Build a proper CORS layer
    let cors = CorsLayer::new()
        // Allow requests from any origin
        .allow_origin(AllowOrigin::any())
        // Allow specific HTTP methods
        .allow_methods(AllowMethods::list([Method::GET, Method::POST]))
        // Allow standard headers
        .allow_headers(AllowHeaders::list([
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ORIGIN,
        ]));

    // Initialize the app router with CORS
    let app = Router::new()
        .route("/get_zodiac", post(get_zodiac_handler))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    // Start the server using hyper and Axum's into_make_service
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct ZodiacRequest {
    dob: String,         // Date of Birth (YYYY-MM-DD)
    skin_tone: String,    // Skin tone: "fair", "medium", "dark"
}

#[derive(Debug, Serialize)]
struct ZodiacResponse {
    zodiac_hindi: String,
    zodiac_english: String,
    lucky_colors: Vec<String>,
}

async fn get_zodiac_handler(Json(payload): Json<ZodiacRequest>) -> Json<ZodiacResponse> {
    let date = match chrono::NaiveDate::parse_from_str(&payload.dob, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            return Json(ZodiacResponse {
                zodiac_hindi: "❌ Invalid Date".to_string(),
                zodiac_english: "❌ Invalid Date".to_string(),
                lucky_colors: vec!["❗ Please use correct format YYYY-MM-DD".to_string()],
            });
        }
    };

    let zodiac = match find_zodiac_sign(date.month() ,date.day()) {
        Some(z) => z,
        None => {
            return Json(ZodiacResponse {
                zodiac_hindi: "❌ Not Found".to_string(),
                zodiac_english: "❌ Not Found".to_string(),
                lucky_colors: vec!["❗ Could not determine zodiac".to_string()],
            });
        }
    };

    let (hindi, english) = zodiac.to_names();
    let colors = get_lucky_colors(zodiac, &payload.skin_tone);

    Json(ZodiacResponse {
        zodiac_hindi: hindi.to_string(),
        zodiac_english: english.to_string(),
        lucky_colors: colors,
    })
}