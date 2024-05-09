use rocket::{Rocket, Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add Cors headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_raw_header("Access-Control-Allow-Origin", "*");
        response.set_raw_header("Access-Control-Allow-Methods", "POST, GET, PUT, DELETE, OPTIONS");
        response.set_raw_header("Access-Control-Allow-Headers", "Authorization, Content-Type");
        response.set_raw_header("Access-Control-Allow-Credentials", "true");
    }
}