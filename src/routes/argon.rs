// use argon2::Config;
// use axum::{
//     extract::Query,
//     http::StatusCode,
//     response::{IntoResponse, Response},
//     routing::get,
//     Router,
// };
// use serde::Deserialize;
// use tracing::error;
//
// pub fn route() -> Router {
//     Router::new().route("/argon", get(argon))
// }
//
// #[derive(Deserialize)]
// struct ArgonParams {
//     password: String,
//     salt: Option<String>,
// }
//
// async fn argon(Query(params): Query<ArgonParams>) -> Response {
//     let password = params.password.as_bytes();
//     let salt = match params.salt {
//         Some(s) => s,
//         None => "samplesalt".to_owned(),
//     };
//
//     let config = Config {
//         variant: argon2::Variant::Argon2id,
//         version: argon2::Version::Version13,
//         mem_cost: 2_u32.pow(15),
//         time_cost: 4,
//         lanes: 2,
//         secret: &[],
//         ad: &[],
//         hash_length: 32,
//     };
//     let hash = match argon2::hash_encoded(password, salt.as_bytes(), &config) {
//         Ok(hash) => hash,
//         Err(e) => {
//             error!("error hashing: {e}");
//             return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
//         }
//     };
//     return (StatusCode::OK, hash).into_response();
// }
