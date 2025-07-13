use rand::random_range;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Seed {
	pub seed: String,
	pub deck: Deck,
	pub stake: Stake,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeedSimple {
	pub seed: String,
	pub deck: String,
	pub stake: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Stake {
	White = 1,
	Red = 2,
	Green = 3,
	Black = 4,
	Blue = 5,
	Purple = 6,
	Orange = 7,
	Gold = 8,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Deck {
	Red,
	Blue,
	Yellow,
	Green,
	Black,
	Magic,
	Nebula,
	Ghost,
	Abandoned,
	Checkered,
	Zodiac,
	Painted,
	Anaglyph,
	Plasma,
	Erratic,
}

pub fn get_random_seed() -> Seed {
	let mut seed: String = String::new();
	while seed.len() < 8 {
		let num: u8 = rand::random_range(0..=35);
		seed.push(match num {
			0..=9 => num.to_string().chars().next().unwrap(),
			_ => (num + 55) as char,
		});
	}
	let deck: Deck;
	deck = match random_range(0..=15) {
		1 => Deck::Red,
		2 => Deck::Blue,
		3 => Deck::Yellow,
		4 => Deck::Green,
		5 => Deck::Black,
		6 => Deck::Magic,
		7 => Deck::Nebula,
		8 => Deck::Ghost,
		9 => Deck::Abandoned,
		10 => Deck::Checkered,
		11 => Deck::Zodiac,
		12 => Deck::Painted,
		13 => Deck::Anaglyph,
		14 => Deck::Plasma,
		15 => Deck::Erratic,
		_ => Deck::Red,
	};
	let stake: Stake;
	stake = match random_range(0..=8) {
		1 => Stake::White,
		2 => Stake::Red,
		3 => Stake::Green,
		4 => Stake::Black,
		5 => Stake::Blue,
		6 => Stake::Purple,
		7 => Stake::Orange,
		8 => Stake::Gold,
		_ => Stake::White,
	};
	return Seed { seed, deck, stake };
}
