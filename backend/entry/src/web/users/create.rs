use axum::{extract::rejection::JsonRejection, Json};

use serde::{Deserialize, Serialize};

use crate::{
    helpers::utils::generate_uuid,
    model::{
        base::Model,
        user::{User, UserTable},
    },
};

use super::errors::{UserError};

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl CreateUserRequest {
    async fn create(self, model: &Model) -> Result<String, UserError> {
        let user_id = generate_uuid();
        let sql_string = UserTable::insert_user(User {
            id: user_id.clone(),
            user_name: self.username,
            email: self.email,
            password: self.password,
        })
        .await
        .map_err(UserError::QueryProblem);

        match sql_string {
            Ok(sql) => match sqlx::query(&sql).execute(&model.db).await {
                Ok(_) => Ok(user_id),
                Err(e) => Err(UserError::DbConnection(e)),
            },
            Err(e) => Err(e),
        }
    }
}

pub async fn handle_create_user(
    payload: Result<Json<CreateUserRequest>, JsonRejection>,
) -> String {
    let model = Model::new().await.unwrap();
    match payload {
        Ok(load) => {
            let create_user = CreateUserRequest {
                username: load.username.clone(),
                password: load.password.clone(),
                email: load.email.clone(),
            }
            .create(&model)
            .await;

            match create_user {
                Ok(user_id) => user_id,
                Err(e) => e.to_string(),
            }
        }
        Err(_) => "Unable to parse JSON".to_string(),
    }
}
