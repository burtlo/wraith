use super::schema::scans;
use diesel::prelude::*;
use diesel;
use diesel::SqliteConnection;

#[table_name = "scans"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Scan {
    pub id: Option<i32>,
    pub data: String
}

impl Scan {
    pub fn create(scan: Scan, connection: &SqliteConnection) -> Scan {
        diesel::insert_into(scans::table)
            .values(&scan)
            .execute(connection)
            .expect(&format!("Error creating the scan {:?}",scan.id));

        scans::table.order(scans::id.desc()).first(connection).unwrap()
    }

    pub fn update(id: i32, scan: Scan, connection: &SqliteConnection) -> bool {
        diesel::update(scans::table.find(id)).set(&scan).execute(connection).is_ok()
    }
}
