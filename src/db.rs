use postgres::{Connection, TlsMode};
use config::Config;


pub fn init(c: &Config) -> Connection {
    let conn = connect(c);
    create_item_table(&conn);
    create_auction_table(&conn);
    conn
}


pub fn connect(c: &Config) -> Connection {
    Connection::connect(c.get_db_url(), TlsMode::None).unwrap()
}

pub fn create_auction_table(conn: &Connection) {
    let sql = "CREATE TABLE IF NOT EXISTS auction (
        auc integer PRIMARY KEY,
        item integer REFERENCES item(id),
        owner VARCHAR(12) NOT NULL,
        ownerRealm VARCHAR(30) NOT NULL,
        bid BIGINT,
        buyout BIGINT,
        quantity smallint,
        timeLeft VARCHAR(15) NOT NULL,
        time timestamp
    )";
    conn.execute(sql, &[]).unwrap();
}

pub fn create_item_table(conn: &Connection) {
    let sql = "CREATE TABLE IF NOT EXISTS item (
        id integer PRIMARY KEY,
        name VARCHAR NOT NULL,
        description VARCHAR NOT NULL,
        stackable smallint,
        icon VARCHAR
    )";
    conn.execute(sql, &[]).unwrap();
}
