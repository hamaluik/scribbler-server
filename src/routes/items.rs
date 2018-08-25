use harsh::Harsh;
use rocket::State;
use rocket_contrib::Json;

use auth::AuthToken;
use communication::ErrorResponses;
use communication::Item;
use db::DbConn;

#[get("/")]
pub fn get_all_items(conn: DbConn, harsh: State<Harsh>, auth: AuthToken) -> Result<Json<Vec<Item>>, ErrorResponses> {
    let mut stmt = conn.prepare("select count(id) from items where owner=?1 limit 1000")
        .expect("prepare select");
    let count: u32 = match stmt.query_row(&[&auth.uid], |row| {
        row.get(0)
    }) {
        Ok(data) => data,
        Err(e) => {
            error!("failed to query database: {:?}", e);
            return Err(ErrorResponses::InternalServerError);
        }
    };

    let mut items: Vec<Item> = Vec::with_capacity(count as usize);
    let mut stmt = conn.prepare("select id, version, content, nonce from items where owner=?1 limit 1000")
        .expect("prepare select");
    let result = stmt.query(&[&auth.uid]);
    match result {
        Ok(mut rows) => {
            while let Some(rowr) = rows.next() {
                match rowr {
                    Ok(row) => {
                        let id: u32 = row.get(0);
                        let hid = harsh.encode(&[id as u64]).expect("harsh encode");

                        let v: i64 = row.get(1);
                        items.push(Item {
                            id: Some(hid),
                            version: v as u64,
                            content: row.get(2),
                            nonce: row.get(3)
                        });
                    }
                    Err(e) => {
                        error!("failed to get row: {:?}", e);
                        return Err(ErrorResponses::InternalServerError);
                    }
                }
            }
        }
        Err(e) => {
            error!("failed to query database: {:?}", e);
            return Err(ErrorResponses::InternalServerError);
        }
    }

    Ok(Json(items))
}

//#[post("/", data="<form>")]
//pub fn upsert_item(form: Json<SignUpForm>, config: State<Config>, conn: DbConn) -> Result<EmptyOK, ErrorResponses> {