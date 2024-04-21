// @generated automatically by Diesel CLI.

diesel::table! {
    students (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 50]
        registration -> Nullable<Varchar>,
        grades -> Nullable<Text>,
    }
}

