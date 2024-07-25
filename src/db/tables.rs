pub static USERS: &str = "
    CREATE TABLE IF NOT EXISTS users (
        id          TEXT NOT NULL UNIQUE PRIMARY KEY,
        name        TEXT NOT NULL UNIQUE,
        pass        TEXT NOT NULL
    )
";

pub static SPLASHES: &str = "
    CREATE TABLE IF NOT EXISTS splashes (
        id          TEXT NOT NULL UNIQUE PRIMARY KEY,
        splash      TEXT NOT NULL
    )
";
