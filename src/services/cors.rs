use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method, Status};
use rocket::{Request, Response};
use std::io::Cursor;

// Code taken from
// https://github.com/SergioBenitez/Rocket/issues/25#issuecomment-271065434
// Might change out for the rocket_cors crate in the future
pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON)
        {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, OPTIONS, PUT",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Access-Control-Allow-Headers, Origin,Accept, X-Requested-With, Content-Type, Access-Control-Request-Method, Access-Control-Request-Headers, Access-Control-Allow-Origin, Authorization"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
            response.set_status(Status::Ok)
        }
    }
}
