use rocket::{
    request::{self, FromRequest},
    Request,
};

#[derive(Debug)]
pub struct AdminOnly {
    pub secret: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminOnly {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        dbg!(request);
        let authorization = match request.headers().get_one("Authorization") {
            Some(v) => v.to_string(),
            None => {
                return rocket::outcome::Outcome::Failure((rocket::http::Status::Unauthorized, ()))
            }
        };

        dbg!(authorization);
        rocket::outcome::Outcome::Success(AdminOnly {
            secret: String::from("secret"),
        })
    }
}
