use actix_web::web;
mod status;
mod user;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    user::configure_user_routes(cfg);
    status::configure_status_routes(cfg);
}
