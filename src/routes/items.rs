use harsh::Harsh;
use rocket::State;
use rocket_contrib::Json;

use auth::AuthToken;
use communication::ErrorResponses;
use communication::Item;
use communication::ID;
use communication::EmptyOK;
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

#[get("/<hid>")]
pub fn get_item(hid: String, harsh: State<Harsh>, conn: DbConn, auth: AuthToken) -> Result<Json<Item>, ErrorResponses> {
    // parse the hid!
    let id: u32 = match harsh.decode(hid) {
        Some(decoded) => decoded[0] as u32,
        None => return Err(ErrorResponses::NotFound)
    };

    let mut stmt = conn.prepare("select id, version, content, nonce from items where id=?1 and owner=?2 limit 1")
        .expect("prepare select");
    let item: Item = match stmt.query_row(&[&id, &auth.uid], |row| {
        let id: u32 = row.get(0);
        let hid = harsh.encode(&[id as u64]).expect("harsh encode");

        Item {
            id: Some(hid),
            version: row.get(1),
            content: row.get(2),
            nonce: row.get(3)
        }
    }) {
        Ok(data) => data,
        Err(e) => {
            error!("failed to query database: {:?}", e);
            return Err(ErrorResponses::NotFound);
        }
    };

    Ok(Json(item))
}

#[post("/", data="<form>")]
pub fn create_item(form: Json<Item>, harsh: State<Harsh>, conn: DbConn, auth: AuthToken) -> Result<Json<ID>, ErrorResponses> {
    let mut stmt = conn.prepare("select count(id) from users")
        .expect("prepare select");
    let count: u32 = match stmt.query_row(&[], |row| {
        row.get(0)
    }) {
        Ok(data) => data,
        Err(e) => {
            error!("failed to query database: {:?}", e);
            return Err(ErrorResponses::InternalServerError);
        }
    };
    debug!("There are {} users!", count);

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
pub fn update_item(id: String, form: Json<Item>, harsh: State<Harsh>, conn: DbConn, auth: AuthToken) -> Result<EmptyOK, ErrorResponses> {
    // it should already exist, we're just updating it
    // parse the hid!
    let id: u32 = match harsh.decode(id) {
        Some(decoded) => decoded[0] as u32,
        None => return Err(ErrorResponses::NotFound)
    };

    // make sure it already exists and we own it
    let mut stmt = conn.prepare("select count(id) from items where id=?1 and owner=?2")
        .expect("prepare select");
    let count: u32 = match stmt.query_row(&[&id, &auth.uid], |row| {
        row.get(0)
    }) {
        Ok(data) => data,
        Err(e) => {
            error!("failed to query database: {:?}", e);
            return Err(ErrorResponses::InternalServerError);
        }
    };
    if count != 1 {
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

#[delete("/<id>")]
pub fn delete_item(id: String, harsh: State<Harsh>, conn: DbConn, auth: AuthToken) -> Result<EmptyOK, ErrorResponses> {
    // it should already exist, we're just updating it
    // parse the hid!
    let id: u32 = match harsh.decode(id) {
        Some(decoded) => decoded[0] as u32,
        None => return Err(ErrorResponses::NotFound)
    };

    // make sure it already exists and we own it
    let mut stmt = conn.prepare("select count(id) from items where id=?1 and owner=?2")
        .expect("prepare select");
    let count: u32 = match stmt.query_row(&[&id, &auth.uid], |row| {
        row.get(0)
    }) {
        Ok(data) => data,
        Err(e) => {
            error!("failed to query database: {:?}", e);
            return Err(ErrorResponses::InternalServerError);
        }
    };
    if count != 1 {
        warn!("tried to update item which does not exist");
        return Err(ErrorResponses::NotFound);
    }

    // delete it
    let mut stmt = conn.prepare("delete from items where id=?1 and owner=?2")
        .expect("prepare statement");
    match stmt.execute(&[&id, &auth.uid]) {
        Ok(affected_rows) => {
            if affected_rows != 1 {
                warn!("failed to delete item! (no changed rows)");
                return Err(ErrorResponses::InternalServerError);
            }
        },
        Err(e) => {
            warn!("failed to delete item: {:?}", e);
            return Err(ErrorResponses::InternalServerError);
        }
    };

    Ok(EmptyOK())
}