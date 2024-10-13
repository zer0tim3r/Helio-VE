// @generated automatically by Diesel CLI.

diesel::table! {
    instance (id) {
        id -> Int4,
        #[max_length = 64]
        uuid -> Varchar,
        label -> Nullable<Text>,
        itype -> Int4,
        image -> Int4,
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

diesel::joinable!(instance -> user (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    instance,
    user,
);
