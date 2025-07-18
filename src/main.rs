use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use profile::profile::{Profile, ProfileManager};

struct AppState {
    profile_manager: ProfileManager,
}

async fn create_profile(data: web::Data<AppState>, profile: web::Json<Profile>) -> impl Responder {
    let manager = &data.profile_manager;
    match manager.create(profile.into_inner()) {
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
    match manager.update(profile.into_inner()) {
        Ok(_) => HttpResponse::Ok().json(manager.get()),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to update profile: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        profile_manager: ProfileManager::new("data/profile.json"),
    });

    println!("Starting server at http://localhost:3005");

    HttpServer::new(move || {
        App::new().app_data(app_state.clone()).service(
            web::resource("/profile")
                .route(web::post().to(create_profile))
                .route(web::get().to(get_profile))
                .route(web::delete().to(delete_profile))
                .route(web::put().to(update_profile)),
        )
    })
    .bind("0.0.0.0:3005")?
    .run()
    .await
}
