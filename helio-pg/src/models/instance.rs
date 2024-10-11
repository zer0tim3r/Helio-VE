use crate::schema::{instance};
use crate::{Timestamptz};
use diesel::prelude::*;
// use macros::DefaultQuery;

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = instance)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Instance {
    pub id: i32,

    pub uuid: String,

    pub label: Option<String>,
    pub params: String,


    pub created_at: Timestamptz,
    pub updated_at: Timestamptz,
}

impl Virtual_Instance {
    pub fn _rpc_create(conn: &mut PgConnection, input: NewUser) -> QueryResult<User> {
        use crate::schema::users::dsl as c;
        let result = diesel::insert_into(c::users)
            .values(&input)
            .get_result::<User>(conn);
        result
    }
}
