use crate::schema::item;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[table_name="item"]
pub struct NewItem<'a> {
    pub name: &'a String,
}
