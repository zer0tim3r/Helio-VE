// @generated automatically by Diesel CLI.

diesel::table! {
    instance (id) {
        id -> Int4,
        #[max_length = 64]
        uuid -> Varchar,
        label -> Nullable<Text>,
        itype -> Int4,
        image -> Int4,
        #[max_length = 20]
        mac -> Varchar,
        #[max_length = 20]
        ipv4 -> Nullable<Varchar>,
        #[max_length = 64]
        created_by -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
