use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
// #[diesel(table_name = crate::schema::users)]
// #[diesel(check_for_backend(diesel::pg::Pg))]

// pub struct UserForCreate {
//     pub id: i32,
//     pub name: String,
//     pub email: String,
//     pub phone: String,
//     pub address: String,
// }
