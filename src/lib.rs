use actix_web::web::ServiceConfig;

pub trait ServiceConfigInitializer {
    fn register_handlers(cfg: &mut ServiceConfig);
}
