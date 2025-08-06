use actix_web::middleware::Logger;
use actix_web::{
    App, Error, HttpResponse, HttpServer, Responder, dev::ServiceRequest, error::ErrorUnauthorized,
    web,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use profile::profile::{Profile, ProfileManager};
use secrecy::{ExposeSecret, SecretString};

struct AppState {
    profile_manager: ProfileManager,
}

async fn create_profile(data: web::Data<AppState>, profile: web::Json<Profile>) -> impl Responder {
    let manager = &data.profile_manager;
    match manager.save(profile.into_inner()) {
        Ok(_) => HttpResponse::Created().json(manager.get()),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to save profile: {}", e))
        }
    }
}

async fn get_profile(data: web::Data<AppState>) -> impl Responder {
    match data.profile_manager.get() {
        Some(c) => HttpResponse::Ok().json(c),
        None => HttpResponse::NotFound().body("No profile found"),
    }
}

async fn delete_profile(data: web::Data<AppState>) -> impl Responder {
    match data.profile_manager.delete() {
        Ok(_) => HttpResponse::Ok().body("Profile deleted successfully"),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete profile: {}", e))
        }
    }
}

async fn update_profile(data: web::Data<AppState>, profile: web::Json<Profile>) -> impl Responder {
    let manager = &data.profile_manager;
    match manager.save(profile.into_inner()) {
        Ok(_) => HttpResponse::Ok().json(manager.get()),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to update profile: {}", e))
        }
    }
}

async fn validate_api_key(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let api_key = req
        .app_data::<web::Data<SecretString>>()
        .expect("API key not found in app data");

    if credentials.token() == api_key.expose_secret() {
        Ok(req)
    } else {
        Err((ErrorUnauthorized("Invalid API key"), req))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        profile_manager: ProfileManager::new("data/profile.json"),
    });

    let api_key =
        SecretString::from(std::env::var("API_KEY").expect("API_KEY environment variable not set"));

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let auth_middleware = HttpAuthentication::bearer(validate_api_key);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .app_data(web::Data::new(api_key.clone()))
            .service(web::resource("/profile").route(web::get().to(get_profile))) // Unprotected
            .service(
                web::resource("/protected")
                    .wrap(auth_middleware.clone())
                    .route(web::post().to(create_profile))
                    .route(web::put().to(update_profile))
                    .route(web::delete().to(delete_profile)),
            )
    })
    .bind("0.0.0.0:3005")?
    .run()
    .await
}
