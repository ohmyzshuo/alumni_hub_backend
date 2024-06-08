use diesel::table;
table! {
    alumnis (id) {
        id -> Int4,
        convocation_year -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        programme -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        matric_no -> Nullable<Varchar>,
        nationality -> Nullable<Varchar>,
        preferences -> Nullable<Jsonb>,
        address -> Nullable<Varchar>,
        occupation -> Nullable<Varchar>,
        location -> Nullable<Varchar>,
        gender -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        workplace -> Nullable<Varchar>,
        supervisor -> Nullable<Array<Text>>,
        thesis_title -> Nullable<Varchar>,
        initial_registration_session -> Nullable<Varchar>,
        faculty_id -> Nullable<Int4>,
        is_first_time_login -> Nullable<Bool>,
        is_hidden -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    staffs (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        is_super_admin -> Nullable<Bool>,
        faculty_id -> Nullable<Int4>,
        phone -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        is_hidden -> Nullable<Bool>,
        gender -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

