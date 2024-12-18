use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::module::{create_jwt, verify_jwt};

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenVerificationRequest {
    pub token: String,
}

#[post("/auth/login")]
async fn login(data: web::Json<AuthRequest>) -> impl Responder {
    // ตรวจสอบ username และ password (ในที่นี้เป็น mock data)
    if data.username == "admin" && data.password == "password" {
        match create_jwt(&data.username) {
            Ok(token) => HttpResponse::Ok().json(AuthResponse { token }),
            Err(err) => HttpResponse::InternalServerError().body(format!("JWT creation error: {}", err)),
        }
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
}

#[post("/auth/verify")]
async fn verify(data: web::Json<TokenVerificationRequest>) -> impl Responder {
    match verify_jwt(&data.token) {
        Ok(claims) => HttpResponse::Ok().json(claims),
        Err(err) => HttpResponse::Unauthorized().body(format!("Token verification error: {}", err)),
    }
}
