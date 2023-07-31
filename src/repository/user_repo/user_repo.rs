use crate::models::user_model::User;
use mongodb::sync::Collection;

pub struct UserRepo {
    pub col: Collection<User>,
}
