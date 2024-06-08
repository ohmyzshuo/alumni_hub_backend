use diesel::table;

table! {
    staff (id) {
        id -> Int4,
        name -> Varchar,
        position -> Varchar,
        department -> Varchar,
    }
}
