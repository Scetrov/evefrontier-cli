use rusqlite::Connection;
use std::path::Path;

fn print_cols(conn: &Connection, table: &str) -> anyhow::Result<()> {
    println!("Schema for {}:", table);
    let mut stmt = conn.prepare(&format!("PRAGMA table_info('{}')", table))?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let cid: i64 = row.get(0)?;
        let name: String = row.get(1)?;
        let typ: Option<String> = row.get(2)?;
        let notnull: i64 = row.get(3)?;
        let dflt: Option<String> = row.get(4)?;
        let pk: i64 = row.get(5)?;
        println!("  {}: {} {} notnull={} pk={} default={}", cid, name, typ.unwrap_or_default(), notnull, pk, dflt.unwrap_or_default());
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let path = Path::new("C:\\Users\\mcp\\AppData\\Local\\evefrontier_datasets\\static_data.db");
    let conn = Connection::open(path)?;
    print_cols(&conn, "SolarSystems")?;
    print_cols(&conn, "Jumps")?;
    Ok(())
}
