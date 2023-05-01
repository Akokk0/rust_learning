use actix_web::web::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>
}

impl From<Json<CreateTeacher>> for CreateTeacher {
    fn from(value: Json<CreateTeacher>) -> Self {
        CreateTeacher {
            name: value.name.clone(),
            picture_url: value.picture_url.clone(),
            profile: value.profile.clone()
        }
    }
}

impl From<Json<UpdateTeacher>> for UpdateTeacher {
    fn from(value: Json<UpdateTeacher>) -> Self {
        UpdateTeacher {
            name: value.name.clone(),
            picture_url: value.picture_url.clone(),
            profile: value.profile.clone()
        }
    }
}