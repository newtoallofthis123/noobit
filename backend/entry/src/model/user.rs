use sea_query::{ColumnDef, Iden, PostgresQueryBuilder, Query, Table};

#[derive(Iden)]
pub enum UserTable {
    Table,
    Id,
    UserName,
    Email,
    Password,
}

pub struct User {
    pub id: String,
    pub user_name: String,
    pub email: String,
    pub password: String,
}

impl UserTable {
    pub fn create_table() -> String {
        Table::create()
            .table(UserTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(UserTable::Id)
                    .not_null()
                    .uuid()
                    .primary_key(),
            )
            .col(ColumnDef::new(UserTable::UserName).string().not_null())
            .col(ColumnDef::new(UserTable::Email).string().not_null().unique_key())
            .col(ColumnDef::new(UserTable::Password).string().not_null())
            .build(PostgresQueryBuilder)
    }

    pub async fn insert_user(user: User) -> Result<String, sea_query::error::Error> {
        match Query::insert()
            .into_table(UserTable::Table)
            .columns(vec![
                UserTable::Id,
                UserTable::UserName,
                UserTable::Email,
                UserTable::Password,
            ])
            .values(vec![
                user.id.into(),
                user.user_name.into(),
                user.email.into(),
                user.password.into(),
            ]) {
            Ok(query) => Ok(query.to_string(PostgresQueryBuilder)),
            Err(e) => Err(e),
        }
    }
}
