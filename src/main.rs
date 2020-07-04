#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::http::{Cookie, Cookies};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;
use sample::schema::words::dsl::*;
use sample::models::{Words, NewWord};
use diesel::prelude::*;

#[derive(Deserialize)]
struct SampleObject {
    word: String
}

fn get_requests_count(mut cookies: Cookies) -> usize {
    // this is a sample of using encrypted cookies to persist session data
    // instead of just in integer you can serialize/deserialize a Json object
    let session = cookies.get_private("session");
    let mut counter = 0;
    if session != None {
        counter = match session.unwrap().value().parse() {
            Ok(counter) => counter,
            Err(e) => {
                println!("{}", e);
                0
            }
        }
    }
    counter += 1;
    cookies.add_private(Cookie::new("session", counter.to_string()));
    counter
}

#[get("/words/<word_id>")]
fn sample_get(cookies: Cookies, word_id: i32) -> JsonValue {
    let connection = sample::establish_connection();
    let rows = words
        .filter(id.eq(word_id))
        .load::<Words>(&connection)
        .expect("Error loading records");
    if rows.len() > 0 {
        json!({
            "word": rows[0].word,
            "requests_count": get_requests_count(cookies)
        })
    } else {
        json!({
            "requests_count": get_requests_count(cookies)
        })
    }
}

#[post("/words", data = "<obj>")]
fn sample_create(cookies: Cookies, obj: Json<SampleObject>) -> JsonValue {
    let connection = sample::establish_connection();
    let new_word = NewWord {
        word: &obj.word,
    };
    let inserted_rows = connection.transaction::<_, diesel::result::Error, _>(|| {
        let inserted_count = diesel::insert_into(words)
        .values(new_word)
        .execute(&connection)?;
        Ok(words
            .order(id.desc())
            .limit(inserted_count as i64)
            .load::<Words>(&connection)?
            .into_iter()
            .rev()
            .collect::<Vec<_>>())
    }).unwrap();
    json!({
        "requests_count": get_requests_count(cookies),
        "inserted_id": inserted_rows[0].id
    })
}

#[get("/words")]
fn sample_retrieve(cookies: Cookies) -> JsonValue {
    let connection = sample::establish_connection();
    let rows = words
        .load::<Words>(&connection)
        .expect("Error loading records");
    #[derive(Serialize)]
    pub struct Record {
        pub my_id: i32,
        pub my_word: String
    }
    let mut items = vec![];
    for row in rows {
        items.push(Record {
            my_id: row.id,
            my_word: row.word
        });
    }
    json!({
        "words": items,
        "requests_count": get_requests_count(cookies)
    })
}

#[put("/words/<word_id>", data = "<obj>")]
fn sample_update(cookies: Cookies, word_id: i32, obj: Json<SampleObject>) -> JsonValue {
    let connection = sample::establish_connection();
    diesel::update(words.find(word_id))
        .set(word.eq(obj.word.clone()))
        .execute(&connection)
        .expect("Error updating record");
    json!({
        "requests_count": get_requests_count(cookies)
    })
}

#[delete("/words/<word_id>")]
fn sample_delete(cookies: Cookies, word_id: i32) -> JsonValue {
    let connection = sample::establish_connection();
    diesel::delete(words.filter(id.eq(word_id)))
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
            sample_get,
            sample_create,
            sample_retrieve,
            sample_update,
            sample_delete,
            sample_http_client
        ]
    )
    .mount("/", StaticFiles::from("./static"))
    .launch();
}
