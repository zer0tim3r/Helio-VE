// @generated automatically by Diesel CLI.

diesel::table! {
    disk (id) {
        id -> Int4,
        #[max_length = 64]
        uuid -> Varchar,
        capacity -> Nullable<Int4>,
        created_by -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    instance (id) {
        id -> Int4,
        #[max_length = 64]
        uuid -> Varchar,
        label -> Nullable<Text>,
        params -> Jsonb,
        created_by -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        #[max_length = 256]
        identifier -> Varchar,
        #[max_length = 256]
        password -> Varchar,
        #[max_length = 64]
        salt -> Varchar,
        crud -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(disk -> user (created_by));
diesel::joinable!(instance -> user (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    disk,
    instance,
    user,
);
