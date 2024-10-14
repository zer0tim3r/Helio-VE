use crate::schema::instance;
use crate::Timestamptz;
use diesel::prelude::*;
// use macros::DefaultQuery;

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = instance)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Instance {
    pub id: i32,

    pub uuid: String,

    pub label: Option<String>,

    pub itype: i32,
    pub image: i32,

    pub mac: String,
    pub ipv4: Option<String>,

    pub created_by: String,
    pub created_at: Timestamptz,
    pub updated_at: Timestamptz,
}

#[derive(Insertable)]
#[diesel(table_name = instance)]
pub struct NewInstance {
    pub uuid: String,

    pub label: Option<String>,

    pub itype: i32,
    pub image: i32,

    pub mac: String,
    pub ipv4: Option<String>,

    pub created_by: String,
}

impl Instance {
    pub fn _default_get_by_id(conn: &mut PgConnection, _id: i32) -> QueryResult<Instance> {
        use crate::schema::instance::dsl::*;

        instance.filter(id.eq(_id)).first(conn)
    }

    pub fn _default_get_by_uuid(
        conn: &mut PgConnection,
        _uuid: String,
        _created_by: String,
    ) -> QueryResult<Instance> {
        use crate::schema::instance::dsl::*;

        instance
            .filter(uuid.eq(_uuid))
            .filter(created_by.eq(_created_by))
            .first(conn)
    }

    pub fn _rpc_list(conn: &mut PgConnection, _created_by: String) -> QueryResult<Vec<Instance>> {
        use crate::schema::instance::dsl::*;

        instance.filter(created_by.eq(_created_by)).get_results(conn)
    }

    pub fn _rpc_create(conn: &mut PgConnection, input: NewInstance) -> QueryResult<Instance> {
        use crate::schema::instance::dsl::*;
        let result = diesel::insert_into(instance)
            .values(&input)
            .get_result::<Instance>(conn);
        result
    }

    pub fn _rpc_delete(
        conn: &mut PgConnection,
        _uuid: String,
        _created_by: String,
    ) -> QueryResult<Instance> {
        use crate::schema::instance::dsl::*;

        diesel::delete(
            instance
                .filter(uuid.eq(_uuid))
                .filter(created_by.eq(_created_by)),
        )
        .get_result::<Instance>(conn)
    }

    pub fn _dhcp_update_ipv4_by_mac(
        conn: &mut PgConnection,
        rmac: String,
        new_ipv4: String,
    ) -> QueryResult<Instance> {
        use crate::schema::instance::dsl::*;

        diesel::update(instance)
            .filter(mac.eq(rmac))
            .set(ipv4.eq(Some(new_ipv4)))
            .get_result(conn)
    }

    pub fn _dhcp_get_by_mac(conn: &mut PgConnection, _mac: String) -> QueryResult<Instance> {
        use crate::schema::instance::dsl::*;

        let query = instance.into_boxed();

        let query = query.filter(mac.eq(_mac));

        query.first(conn)
    }

    pub fn _dhcp_get_by_ipv4(conn: &mut PgConnection, _ipv4: String) -> QueryResult<Instance> {
        use crate::schema::instance::dsl::*;

        let query = instance.into_boxed();

        let query = query.filter(ipv4.eq(Some(_ipv4)));

        query.first(conn)
    }
}
