use anyhow::{Context, Result};
use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct System {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Jump {
    pub from_id: i64,
    pub to_id: i64,
}

/// Load systems and gate jumps from the starmap SQLite database.
/// The exact table/column names here assume EVE-style static data;
/// adjust if your schema differs.
pub fn load_starmap(path: &std::path::Path) -> Result<(Vec<System>, Vec<Jump>)> {
    let conn = Connection::open(path)
        .with_context(|| format!("failed to open SQLite database {}", path.display()))?;

    let mut systems = Vec::new();
    let mut jumps = Vec::new();

    {
        // static_data.db uses `SolarSystems` with columns `solarSystemId` and `name`.
        let mut stmt = conn.prepare(
            "SELECT solarSystemId, name
             FROM SolarSystems",
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            systems.push(System {
                id: row.get(0)?,
                name: row.get(1)?,
            });
        }
    }

    {
        // static_data.db uses `Jumps` with columns `fromSystemId` and `toSystemId`.
        let mut stmt = conn.prepare(
            "SELECT fromSystemId, toSystemId
             FROM Jumps",
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            jumps.push(Jump {
                from_id: row.get(0)?,
                to_id: row.get(1)?,
            });
        }
    }

    Ok((systems, jumps))
}
