pub mod handler{
    use actix_web::{get, Responder};

    #[get("/kapi")]
    pub async fn ping_handler() -> impl Responder {
        "hello kapi"
    }
}