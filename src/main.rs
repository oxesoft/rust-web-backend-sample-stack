#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::http::{Cookie, Cookies};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;
use sample::schema::item::dsl::*;
use sample::models::{Item, NewItem};
use diesel::prelude::*;

#[derive(Deserialize)]
struct SampleObject {
    name: String
}

fn get_requests_count(mut cookies: Cookies) -> usize {
    #[derive(Serialize, Deserialize)]
    struct SessionObject {
        counter: usize
    }
    let session = cookies.get_private("session");
    let mut session_object = SessionObject {
        counter: 0
    };
    if session != None {
        session_object = serde_json::from_str(session.unwrap().value()).unwrap();
    }
    session_object.counter += 1;
    let session_object_string = match serde_json::to_string(&session_object) {
        Ok(session_object_string) => session_object_string,
            Err(e) => {
                println!("{}", e);
                format!("{{\"error\":{}}}", e)
            }
    };
    cookies.add_private(Cookie::new("session", session_object_string));
    session_object.counter
}

#[get("/items")]
fn sample_retrieve_all(cookies: Cookies) -> JsonValue {
    let connection = sample::establish_connection();
    let rows = item
        .load::<Item>(&connection)
        .expect("Error loading records");
    #[derive(Serialize)]
    pub struct Record {
        pub my_id: i32,
        pub my_name: String
    }
    let mut items = vec![];
    for row in rows {
        items.push(Record {
            my_id: row.id,
            my_name: row.name
        });
    }
    json!({
        "items": items,
        "requests_count": get_requests_count(cookies)
    })
}

#[post("/items", data = "<obj>")]
fn sample_create(cookies: Cookies, obj: Json<SampleObject>) -> JsonValue {
    let connection = sample::establish_connection();
    let new_item = NewItem {
        name: &obj.name,
    };
    let inserted_rows = connection.transaction::<_, diesel::result::Error, _>(|| {
        let inserted_count = diesel::insert_into(item)
        .values(new_item)
        .execute(&connection)?;
        Ok(item
            .order(id.desc())
            .limit(inserted_count as i64)
            .load::<Item>(&connection)?
            .into_iter()
            .rev()
            .collect::<Vec<_>>())
    }).unwrap();
    json!({
        "requests_count": get_requests_count(cookies),
        "inserted_id": inserted_rows[0].id
    })
}

#[get("/items/<item_id>")]
fn sample_retrieve(cookies: Cookies, item_id: i32) -> JsonValue {
    let connection = sample::establish_connection();
    let rows = item
        .filter(id.eq(item_id))
        .load::<Item>(&connection)
        .expect("Error loading records");
    if rows.len() > 0 {
        json!({
            "name": rows[0].name,
            "requests_count": get_requests_count(cookies)
        })
    } else {
        json!({
            "requests_count": get_requests_count(cookies)
        })
    }
}

#[put("/items/<item_id>", data = "<obj>")]
fn sample_update(cookies: Cookies, item_id: i32, obj: Json<SampleObject>) -> JsonValue {
    let connection = sample::establish_connection();
    diesel::update(item.find(item_id))
        .set(name.eq(obj.name.clone()))
        .execute(&connection)
        .expect("Error updating record");
    json!({
        "requests_count": get_requests_count(cookies)
    })
}

#[delete("/items/<item_id>")]
fn sample_delete(cookies: Cookies, item_id: i32) -> JsonValue {
    let connection = sample::establish_connection();
    diesel::delete(item.filter(id.eq(item_id)))
        .execute(&connection)
        .expect("Error deleting record");
    json!({
        "requests_count": get_requests_count(cookies)
    })
}

#[get("/myip")]
fn sample_http_client(cookies: Cookies) -> JsonValue {
    fn get_ip() -> Result<String, Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct Ip {
            origin: String
        }
        let json: Ip = reqwest::blocking::get("http://httpbin.org/ip")?.json()?;
        Ok(json.origin)
    }
    let result = get_ip();
    json!({
        "myIP": result.unwrap(),
        "requests_count": get_requests_count(cookies)
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn main() {
    rocket::ignite()
    .register(
        catchers![
            not_found
        ]
    )
    .mount("/",
        routes![
            sample_retrieve_all,
            sample_create,
            sample_retrieve,
            sample_update,
            sample_delete,
            sample_http_client
        ]
    )
    .mount("/", StaticFiles::from("./public"))
    .launch();
}
