use crate::models::staff::{ NewStaff, Staff };
use crate::schema::staff::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;

/// Creates a new staff member in the database.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `new_staff` - The new staff member to be created.
///
/// # Returns
///
/// Returns a `QueryResult` containing the newly created staff member, or an error if the insertion fails.
pub fn create_staff(
    conn: &mut PgConnection,
    new_staff: NewStaff
) -> QueryResult<Staff, diesel::result::Error> {
    diesel::insert_into(staff).values(&new_staff).get_result(conn)
}

pub fn get_staff(conn: &mut PgConnection, staff_id: i32) -> QueryResult<Staff> {
    staff.find(staff_id).get_result(conn)
}
