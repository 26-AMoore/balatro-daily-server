use chooser::{get_random_seed, Seed, SeedSimple};
use db::{init_db, init_runs, init_seed, Run};
use sqlx::{sqlite, Pool, Sqlite};
use std::{
	error::Error,
	sync::mpsc::{self, Receiver, Sender},
};
use tokio::{self, join};
use warp::{
	filters::{body::json, query::query},
	reply::Reply,
	Filter,
};
mod chooser;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	println!("{:?}", get_random_seed());
	//pretty_env_logger::init();
	let address = [0, 0, 0, 0];

	let (sender, receiver) = mpsc::channel::<String>();

	let pool = init_runs().await;
	let seed_pool = init_seed().await;
	let seed = get_seed(seed_pool).await;

	let seed_filter = warp::any().and(warp::path("seed").map(move || warp::reply::json(&seed)));
	let post_filter = warp::post()
		.and(warp::path("upload"))
		.and(json())
		.map(move |data: Run| handle_run_upload(data, sender.clone()));

	let post_serve = warp::serve(post_filter.or(seed_filter)).run((address, 3030));
	_ = join!(
		tokio::spawn(post_serve),
		handle_mpsc_receiver(receiver, pool)
	);
	Ok(())
}

async fn handle_mpsc_receiver(receiver: Receiver<String>, pool: Pool<Sqlite>) {
	for query in receiver.iter() {
		println!("{}", query);
		println!("{:?}", sqlx::query(&query).execute(&pool).await);
	}
}

async fn get_seed(pool: Pool<Sqlite>) -> SeedSimple {
	let seed = sqlx::query!("select * from seed",)
		.fetch_one(&pool)
		.await
		.unwrap();
	SeedSimple {
		seed: seed.seed.unwrap(),
		deck: seed.deck.unwrap(),
		stake: seed.stake.unwrap(),
	}
}

fn handle_run_upload(data: Run, sender: Sender<String>) -> impl Reply {
	let query: String = format!(
		r#"INSERT INTO runs (id,name,ante,round,best_hand,rerolls,endless)
VALUES("{}","{}",{},{},{},{},{});"#,
		data.id, data.name, data.ante, data.round, data.best_hand, data.rerolls, data.endless
	);
	sender.send(query).ok();
	println!("{:?}", data);
	warp::reply()
}
