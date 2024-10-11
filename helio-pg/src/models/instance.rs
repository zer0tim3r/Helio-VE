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
    pub params: serde_json::Value,


    pub created_at: Timestamptz,
    pub updated_at: Timestamptz,
}

#[derive(Insertable)]
#[diesel(table_name = instance)]
pub struct NewInstance {
    pub uuid: String,

    pub label: Option<String>,
    pub params: serde_json::Value,

    pub created_at: Timestamptz,
    pub updated_at: Timestamptz,
}

impl Instance {
    pub fn _rpc_create(conn: &mut PgConnection, input: NewInstance) -> QueryResult<Instance> {
        use crate::schema::instance::dsl::*;
        let result = diesel::insert_into(instance)
            .values(&input)
            .get_result::<Instance>(conn);
        result
    }
}
