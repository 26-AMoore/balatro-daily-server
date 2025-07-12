use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, sqlx::Type)]
pub struct Run {
	pub id: String,
	pub name: String,
	pub ante: i8,
	pub round: i16,
	pub best_hand: f64,
	pub rerolls: u32,
	pub endless: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Leaderboard {
	Ante,
	Round,
	BestHand,
	TimesRerolled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Leaderboards {
	field: Leaderboard,
	endless: bool,
}

pub async fn init_db(database_url: &str) -> Option<Pool<Sqlite>> {
	if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
		match Sqlite::create_database(database_url).await {
			Ok(_) => eprintln!("Database created"),
			Err(e) => eprintln!("Error creating database: {}", e),
		}
	}
	sqlx::sqlite::SqlitePool::connect(database_url).await.ok()
}
