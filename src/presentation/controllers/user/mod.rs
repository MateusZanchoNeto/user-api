use actix_web::web;

mod create_user;
mod delete_user;
mod get_user;
mod get_users;
mod update_user;

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user::get_user);
    cfg.service(create_user::create_user);
    cfg.service(delete_user::delete_user);
    cfg.service(get_users::get_users);
    cfg.service(update_user::update_user);
}
