use rusqlite::Connection;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let path = Path::new("C:\\Users\\mcp\\AppData\\Local\\evefrontier_datasets\\static_data.db");
    let conn = Connection::open(path)?;
    let mut stmt = conn.prepare("SELECT name, type FROM sqlite_master WHERE type IN ('table','view') ORDER BY name")?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        let kind: String = row.get(1)?;
        println!("{} ({})", name, kind);
    }
    Ok(())
}
