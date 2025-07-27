use chooser::SeedSimple;
use clap::Parser;
use db::{init_runs, init_seed, Run};
use sqlx::{Pool, Row, Sqlite};
use std::{
	collections::HashSet,
	error::Error,
	sync::mpsc::{self, Receiver, Sender},
	time::Instant,
};
use tokio::{self, join};
use warp::{filters::body::json, reply::Reply, Filter};
mod chooser;
mod db;
mod tests;
use tests::populate_random;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	//pretty_env_logger::init();
	let args = Args::parse();
	if args.clear {
		println!("cleaning");
		db::clean(&init_runs().await).await;
		init_seed().await;
		return Ok(());
	}
	if args.tests > 0 {
		populate_random(args.tests).await;
	}

	let address = [0, 0, 0, 0];

	let (run_sender, run_receiver) = mpsc::channel::<String>();
	let mut ids: HashSet<String> = HashSet::new();

	let pool = init_runs().await;
	for id in sqlx::query!("select id from runs;")
		.fetch_all(&pool)
		.await
		.unwrap()
		.iter()
	{
		ids.insert(id.id.clone().unwrap());
	}
	let seed_pool = init_seed().await;
	let seed = get_seed(seed_pool).await;

	//filters
	let id_filter = warp::any().and(warp::path("id").and(warp::path::param()).and_then(
		|id: String| async move {
			let pool = init_runs().await;
			let data = sqlx::query(&format!("select id from runs where id={}", id))
				.fetch_one(&pool)
				.await;
			if true {
				Ok(match data {
					Ok(_) => warp::reply::html("true"),
					Err(_) => warp::reply::html("false"),
				})
			} else {
				Err(warp::reject())
			}
		},
	));
	let version_filter = warp::any().and(warp::path("version").map(|| warp::reply::html(r#"0"#)));
	let leaderboard_filter = warp::any().and(
		warp::path("leaderboards")
			.and(warp::path::param())
			.and_then(async move |page: String| get_leaderboard(page).await),
	);
	let seed_filter = warp::any().and(warp::path("seed").map(move || warp::reply::json(&seed)));
	let post_filter = warp::post()
		.and(warp::path("upload"))
		.and(json())
		.map(move |data: Run| handle_run_upload(data, run_sender.clone()));

	let serve = warp::serve(
		post_filter
			.or(seed_filter)
			.or(version_filter)
			.or(id_filter)
			.or(leaderboard_filter),
	)
	.run((address, 3030));
	_ = join!(
		tokio::spawn(serve),
		handle_run_mpsc_receiver(run_receiver, pool),
	);

	Ok(())
}

async fn handle_run_mpsc_receiver(receiver: Receiver<String>, pool: Pool<Sqlite>) {
	for query in receiver.iter() {
		println!("{}", query);
		println!("{:?}", sqlx::query(&query).execute(&pool).await);
	}
}

async fn get_seed(pool: Pool<Sqlite>) -> SeedSimple {
	let seed = sqlx::query!("select * from seed")
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

async fn get_leaderboard(
	page: String,
) -> Result<warp::reply::Html<String>, warp::reject::Rejection> {
	let pool = init_runs().await;
	let page: usize = str::parse(&page).unwrap();

	// we have json library at home, json lib at home
	let mut fake_json: Vec<String> = Vec::new();
	fake_json.push("{".to_string());
	for i in 0..10 {
		match sqlx::query(&format!(
			"select name,best_hand from runs order by best_hand desc limit 1 offset {};",
			page * 10 + i
		))
		.fetch_one(&pool)
		.await
		{
			// I am sorry
			Ok(data) => {
				fake_json.push(String::from(",\"name\":\""));
				fake_json.push(data.get(0));
				fake_json.push("\",\"score\":\"".to_string());
				let score: f64 = data.get(1);
				fake_json.push(score.to_string());
				fake_json.push("\"".to_string());
			}
			Err(_) => (),
		};
	}

	if true {
		fake_json.push("}".to_string());
		let name: String = fake_json.concat();
		println!("{}", name);
		Ok(warp::reply::html(name))
	} else {
		Err(warp::reject())
	}
}
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// clears the database
	#[arg(short, long)]
	clear: bool,
	/// how many random values to add into the database for testing
	#[arg(short, long, default_value = "0")]
	tests: i64,
}
