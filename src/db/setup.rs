use errors::Error;
use db::pool::DatabasePool;

pub fn initialize_tables(pool: &DatabasePool) -> Result<(), Error> {
    let conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return Err(Error::PoolError)
    };

    if let Err(e) = conn.execute("create table if not exists users (
            id integer primary key,
            name text not null,
            server_key text not null,
            salt text not null
        )", &[]) {
        return Err(Error::DatabaseError(e));
    }

    if let Err(e) = conn.execute("create table if not exists items (
            id integer primary key,
            owner integer not null,
            version int not null,
            content text not null,
            nonce text not null,
            foreign key (owner) references users(id)
        )", &[]) {
        return Err(Error::DatabaseError(e));
    }

    /*if let Err(e) = conn.execute("create table if not exists items_shared (
            id integer primary key,
            owner integer not null
            version int not null,
            content text not null,
            salt text not null,
            nonce text not null,
            foreign key (owner) references users(id)
        )", &[]) {
        return Err(Error::DatabaseError(e));
    }*/

    Ok(())
}

/*pub fn register_user(pool: &DatabasePool, name: &str, server_key: &str, salt: &str) -> Result<u32, Error> {
    let conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return Err(Error::PoolError)
    };

    conn.execute(
        "insert into users(name, server_key, salt) values(?1, ?2, ?3)",
        &[&name, &server_key, &salt]
    )?;

    let mut stmt = conn.prepare("select id from users order by id desc limit 1")?;
    let id:u32 = stmt.query_row(&[], |row| row.get(0))?;
    println!("created used with id: {}", id);

    Ok(id)
}*/
