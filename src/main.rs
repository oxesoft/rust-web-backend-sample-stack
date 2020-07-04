#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;
use sample::schema::words::dsl::*;
use sample::models::{Words, NewWord};
use diesel::prelude::*;

#[derive(Default, Clone)]
struct SessionData {
    requests_count: usize,
}

type Session<'a> = rocket_session::Session<'a, SessionData>;

#[derive(Deserialize)]
struct SampleObject {
    word: String
}

fn get_requests_count(session: Session) -> usize {
    let value = session.tap(|sess| {
        sess.requests_count += 1;
        sess.requests_count
    });
    value
}

#[get("/words/<word_id>")]
fn sample_get(session: Session, word_id: i32) -> JsonValue {
    let connection = sample::establish_connection();
    let rows = words
        .filter(id.eq(word_id))
        .load::<Words>(&connection)
        .expect("Error loading records");
    if rows.len() > 0 {
        return json!({
            "word": rows[0].word,
            "requests_count": get_requests_count(session)
        })
    }
    json!({})
}

#[post("/words", data = "<obj>")]
fn sample_create(session: Session, obj: Json<SampleObject>) -> JsonValue {
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
        "requests_count": get_requests_count(session),
        "inserted_id": inserted_rows[0].id
    })
}

#[get("/words")]
fn sample_retrieve(session: Session) -> JsonValue {
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
        "requests_count": get_requests_count(session)
    })
}

#[put("/words/<word_id>", data = "<obj>")]
fn sample_update(session: Session, word_id: i32, obj: Json<SampleObject>) -> JsonValue {
    let connection = sample::establish_connection();
    diesel::update(words.find(word_id))
        .set(word.eq(obj.word.clone()))
        .execute(&connection)
        .expect("Error updating record");
    json!({
        "requests_count": get_requests_count(session)
    })
}

#[delete("/words/<word_id>")]
fn sample_delete(session: Session, word_id: i32) -> JsonValue {
    let connection = sample::establish_connection();
    diesel::delete(words.filter(id.eq(word_id)))
        .execute(&connection)
        .expect("Error deleting record");
    json!({
        "requests_count": get_requests_count(session)
    })
}

#[get("/myip")]
fn sample_http_client(session: Session) -> JsonValue {
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
        "requests_count": get_requests_count(session)
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
    .attach(Session::fairing()) 
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
