use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    data::Outcome,
    http::Status,
    request::FromRequest,
    serde::{Deserialize, Serialize},
    Request,
};
use std::{env, error::Error, time};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct Jwt {
    secret: String,
}

impl Jwt {
    pub fn init() -> Self {
        dotenv().ok();

        let secret = match env::var("JWT_SECRET") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error, variável nao encontrada"),
        };

        Self { secret }
    }

    pub fn encode(&self, id: &String) -> Result<String, Box<dyn Error>> {
        let expiration = time::Duration::from_secs(60 * 60 * 24 * 2); // 2 dias
        let expiration = time::SystemTime::now()
            .checked_add(expiration)
            .ok_or("Error to generate token: Invalid expiration time")?
            .duration_since(time::UNIX_EPOCH)
            .map_err(|_| "Error to generate token: Invalid expiration time")?;

        let expiration = expiration.as_secs() as usize;

        let claims = Claims {
            exp: expiration,
            sub: id.to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|_| "Error to generate token")?;

        println!("{}", token);

        Ok(token)
    }

    pub fn decode(&self, token: &String) -> Result<String, Box<dyn Error>> {
        let token = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| "Error to decode token")?;

        Ok(token.claims.sub)
    }
}
