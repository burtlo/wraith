use super::schema::scans;

#[table_name = "scans"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Scan {
    pub id: Option<i32>,
    pub data: String
}
