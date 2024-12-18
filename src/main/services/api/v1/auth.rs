use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::module::{create_jwt, verify_jwt};
use crate::module::jwt_auth::{Role, UserClaims};

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
    // Mock data: ตรวจสอบ username และ password
    let role = if data.username == "admin" && data.password == "password" {
        Role::Admin
    } else if data.username == "user" && data.password == "password" {
        Role::BaseUser
    } else {
        return HttpResponse::Unauthorized().body("Invalid username or password");
    };

    // สร้าง JWT พร้อม role
    let user_claims = UserClaims {
        id: 1, // mock user ID
        role,
    };

    match create_jwt(&user_claims) {
        Ok(token) => HttpResponse::Ok().json(AuthResponse { token }),
        Err(err) => HttpResponse::InternalServerError().body(format!("JWT creation error: {}", err)),
    }
}

#[post("/auth/verify")]
async fn verify(data: web::Json<TokenVerificationRequest>) -> impl Responder {
    match verify_jwt(&data.token) {
        Ok(user_claims) => {
            // เรียกใช้งาน verify_service_request เพื่อตรวจสอบสิทธิ์
            if verify_service_request(user_claims.clone()).await {
                HttpResponse::Ok().json(user_claims)
            } else {
                HttpResponse::Forbidden().body("You do not have permission to access this resource.")
            }
        }
        Err(err) => HttpResponse::Unauthorized().body(format!("Token verification error: {}", err)),
    }
}


pub async fn verify_service_request(user_claims: UserClaims) -> bool {
    match user_claims.role {
        Role::Admin => true,
        Role::BaseUser => false,
    }
}
