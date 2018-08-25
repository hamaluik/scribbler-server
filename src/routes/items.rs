use harsh::Harsh;
use rocket::State;
use rocket_contrib::Json;

use auth::AuthToken;
use communication::ErrorResponses;
use communication::Item;
use communication::ID;
use communication::EmptyOK;
use db::DbConn;
use config::Config;

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

                        items.push(Item {
                            id: Some(hid),
                            version: row.get(1),
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

#[post("/", data="<form>")]
pub fn create_item(form: Json<Item>, harsh: State<Harsh>, config: State<Config>, conn: DbConn, auth: AuthToken) -> Result<Json<ID>, ErrorResponses> {
    // insert it
    let mut stmt = conn.prepare("insert into items(owner, version, content, nonce) values(?1, ?2, ?3, ?4)")
        .expect("prepare statement");
    match stmt.execute(&[&auth.uid, &form.version, &form.content, &form.nonce]) {
        Ok(affected_rows) => {
            if affected_rows != 1 {
                warn!("failed to insert item! (no changed rows)");
                return Err(ErrorResponses::InternalServerError);
            }
        },
        Err(e) => {
            warn!("failed to insert item: {:?}", e);
            return Err(ErrorResponses::InternalServerError);
        }
    };

    // query it
    let mut stmt = conn.prepare("select id from items where owner=?1 order by id desc limit 1")
        .expect("prepare select");
    let id: u32 = match stmt.query_row(&[&auth.uid], |row| {
        row.get(0)
    }) {
        Ok(data) => data,
        Err(e) => {
            error!("failed to query database: {:?}", e);
            return Err(ErrorResponses::InternalServerError);
        }
    };

    // return it
    let hid = harsh.encode(&[id as u64]).expect("harsh encode");
    Ok(Json(ID { id: hid }))
}

#[patch("/<id>", data="<form>")]
pub fn update_item(id: String, form: Json<Item>, harsh: State<Harsh>, config: State<Config>, conn: DbConn, auth: AuthToken) -> Result<EmptyOK, ErrorResponses> {
    // it should already exist, we're just updating it
    // parse the hid!
    let id: u32 = match harsh.decode(id) {
        Some(decoded) => decoded[0] as u32,
        None => return Err(ErrorResponses::NotFound)
    };

    // make sure it already exists and we own it
    let mut stmt = conn.prepare("select count(id), owner from items where id=?1")
        .expect("prepare select");
    let result: (u32, u32) = match stmt.query_row(&[&id], |row| {
        (row.get(0), row.get(1))
    }) {
        Ok(data) => data,
        Err(e) => {
            error!("failed to query database: {:?}", e);
            return Err(ErrorResponses::InternalServerError);
        }
    };
    if result.1 != auth.uid {
        warn!("tried to edit item which didn't own");
        return Err(ErrorResponses::Unauthorized);
    }
    if result.0 != 1 {
        warn!("tried to update item which does not exist");
        return Err(ErrorResponses::NotFound);
    }

    // update it
    let mut stmt = conn.prepare("update items set version=?1, content=?2, nonce=?3 where id=?4")
        .expect("prepare statement");
    match stmt.execute(&[&form.version, &form.content, &form.nonce, &id]) {
        Ok(affected_rows) => {
            if affected_rows != 1 {
                warn!("failed to update item! (no changed rows)");
                return Err(ErrorResponses::InternalServerError);
            }
        },
        Err(e) => {
            warn!("failed to update item: {:?}", e);
            return Err(ErrorResponses::InternalServerError);
        }
    };

    Ok(EmptyOK())
}