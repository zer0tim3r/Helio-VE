// @generated automatically by Diesel CLI.

diesel::table! {
    disk (id) {
        id -> Int4,
        #[max_length = 64]
        uuid -> Varchar,
        capacity -> Nullable<Int4>,
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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    disk,
    instance,
);
