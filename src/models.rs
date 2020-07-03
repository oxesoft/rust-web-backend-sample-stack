use crate::schema::words;

#[derive(Queryable)]
pub struct Words {
    pub id: i32,
    pub word: String
}

#[derive(Insertable)]
#[table_name="words"]
pub struct NewWord<'a> {
    pub word: &'a String,
}
