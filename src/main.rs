use db::{init_db, Run};
use sqlx::{sqlite, Pool, Sqlite};
use std::{
	error::Error,
	sync::mpsc::{self, Receiver},
};
use tokio::{self, join};
use warp::{
	filters::{body::json, query::query},
	Filter,
};
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	//pretty_env_logger::init();
	let address = [0, 0, 0, 0];

	let (sender, receiver) = mpsc::channel::<String>();

	let pool = match init_db("db/db.sqlite").await {
		Some(p) => p,
		None => panic!("no db"),
	};

	println!(
		"{:?}",
		sqlx::query(
			"CREATE TABLE runs (
id TEXT,
name TEXT,
ante INTEGER,
round INTEGER,
best_hand REAL,
rerolls INTEGER,
endless BOOLEAN
    );",
		)
		.execute(&pool)
		.await
	);

	let post = warp::post()
		.and(warp::path("upload"))
		.and(json())
		.map(move |data: Run| {
			let query: String = format!(
				r#"INSERT INTO runs (id,name,ante,round,best_hand,rerolls,endless)
VALUES("{}","{}",{},{},{},{},{});"#,
				data.id,
				data.name,
				data.ante,
				data.round,
				data.best_hand,
				data.rerolls,
				data.endless
			);
			sender.send(query).ok();
			println!("{:?}", data);
			warp::reply()
		});

	let post_serve = warp::serve(post).run((address, 3030));
	_ = join!(tokio::spawn(post_serve), handle_request(receiver, pool));
	Ok(())
}

async fn handle_request(receiver: Receiver<String>, pool: Pool<Sqlite>) {
	for query in receiver.iter() {
		println!("{}", query);
		println!("{:?}", sqlx::query(&query).execute(&pool).await);
	}
}
