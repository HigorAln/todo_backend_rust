use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{
    response::status::Custom,
    serde::{json::Json, Deserialize, Serialize},
};
use todo_backend::ResponseError;

#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
    title: String,
    description: String,
    owner: ObjectId,
}

// #[post("/", data = "<todo>")]
// pub fn create_todo(
//     todo: Json<RequestBody>,
// ) -> Result<Json<InsertOneResult>, Custom<Json<ResponseError>>> {
//     let collection = TodoRepo::init();
//     // let todo = collection.create_todo(todo.title, todo.description, todo.owner.to_string());

//     match todo {
//         Ok(todo) => Ok(Json(todo)),
//         Err(err) => Err(Custom(err.status.unwrap(), Json(err))),
//     }
// }
