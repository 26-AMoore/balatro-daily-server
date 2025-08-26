use rand::random;

use crate::db::{self, Run};

pub async fn populate_random(amount: i64) {
	let pool = db::init_db("db/db.sqlite").await;
	for i in 0..amount {
		let data = Run {
			id: 1.to_string(), // random::<i64>().to_string(),
			name: i.to_string(),
			ante: random(),
			round: i as i16,
			best_hand: rand::random::<u64>() as f64,
			rerolls: i as u32,
			endless: false,
		};

		let query: String = format!(
			r#"INSERT INTO runs (id,name,ante,round,best_hand,rerolls,endless)
        VALUES("{}","{}",{},{},{},{},{});"#,
			data.id, data.name, data.ante, data.round, data.best_hand, data.rerolls, data.endless
		);
		sqlx::query(&query).execute(&pool).await;
	}

	//	let mut curl = std::process::Command::new("curl");
	//	let mut data: String;
	//	data =
	//r#"'{"id":"123123123","ante":10, "round":10, "best_hand":1e301,"rerolls":100,"endless":false,"name":"Xela"}'"#
	//.to_string();
	//	let res = curl
	//		.arg("127.0.0.1:3030/upload")
	//		.arg("-X")
	//		.arg("POST")
	//		.arg("-d")
	//		.arg(&data)
	//		.arg("-H")
	//		.arg(r#""content-type:application/json""#)
	//		.output();
	//    println!("{:?}", res);
}
