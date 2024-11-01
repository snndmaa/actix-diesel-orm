// @generated automatically by Diesel CLI.

diesel::table! {
    article (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        created_by -> Nullable<Int4>,
        created_on -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
    }
}

diesel::joinable!(article -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    article,
    users,
);
