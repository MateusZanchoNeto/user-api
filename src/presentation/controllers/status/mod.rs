use actix_web::web;

mod get_status;

pub fn configure_status_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_status::get_status);
}
