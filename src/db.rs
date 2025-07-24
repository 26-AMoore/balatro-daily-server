use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite};
use warp::{filters::query::query, reply::reply};

use crate::chooser;

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

pub async fn init_runs() -> Pool<Sqlite> {
	let db = init_db("db/db.sqlite").await;

	println!(
		"{:?}",
		sqlx::query(
			"CREATE TABLE IF NOT EXISTS runs (
                id TEXT,
                name TEXT,
                ante INTEGER,
                round INTEGER,
                best_hand REAL,
                rerolls INTEGER,
                endless BOOLEAN
            );",
		)
		.execute(&db)
		.await
	);
	db
}

pub async fn init_seed() -> Pool<Sqlite> {
	let db = init_db("db/db.sqlite").await;
	if sqlx::query!("select * from seed;")
		.fetch_one(&db)
		.await
		.is_ok()
	{
		return db;
	}
	println!(
		"making seed table: {:?}",
		sqlx::query(
			//hackyyyyyyy
			"CREATE TABLE IF NOT EXISTS seed (
                seed TEXT,
                deck TEXT,
                stake INTEGER
            );",
		)
		.execute(&db)
		.await
	);
	let seed = chooser::get_random_seed();
	println!("{:?}", seed);
	let request = format!(
		r#"INSERT INTO seed (seed,deck,stake)
            VALUES("{}","{:?}",{})"#,
		&seed.seed, seed.deck, seed.stake as u8
	);

	println!(
		"inserting seed: {:?}",
		sqlx::query(&request).execute(&db).await
	);

	db
}

pub async fn init_db(database_url: &str) -> Pool<Sqlite> {
	if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
		match Sqlite::create_database(database_url).await {
			Ok(_) => eprintln!("Database created"),
			Err(e) => eprintln!("Error creating database: {}", e),
		}
	}
	sqlx::sqlite::SqlitePool::connect(database_url)
		.await
		.unwrap()
}

pub async fn clean(pool: &Pool<Sqlite>) {
	_ = sqlx::query!("delete from runs;").execute(pool).await;
	_ = sqlx::query!("delete from seed;").execute(pool).await;
}
