// Not in the standard library, note that I'm pulling this package from the 
// web via crates.io, it needed to be listed in Cargo.toml dependecies
use rand::Rng;

// Can be used like classic enum:
enum Color {
	Red,
	Green,
	Blue,
}

// Can be used to solve the lack of null
fn maybe_get_color() -> Option<Color> {
	let chance = rand::thread_rng().gen_range(0..100);

	println!("Generated: {}", chance);

	// match is an expression
	match chance {
		0..=24 => Some(Color::Red), // Option::Some, but included by default
		25..=49 => Some(Color::Green),
		50..=74 => Some(Color::Blue),
		_ => None,    // Option::None, but included by default
	}
}

fn main() {
	let maybe_col = maybe_get_color();
	match maybe_col {
		// Can provide a pattern to match and destructure the Option returned
		Some(c) => match c {
			Color::Red => println!("Got red!"),
			Color::Green => println!("Got green!"),
			Color::Blue => println!("Got blue!"),
		},
		None => println!("Got nothing :("),
	}

	let maybe_shot = maybe_get_shot();
	match maybe_shot {
		Some(s) => match s {
			Shot::Two => println!("Made a two pointer!"),
			Shot::Three => println!("Made a three!"),
			Shot::FreeThrow => print!("Made a free throw!"),
		},
		None => println!("You missed the shot :(")
	}
}

//TODO: Write your own enum, and use it with another match statement

//<Your code here>
enum Shot {
	Three,
	Two,
	FreeThrow
}

fn maybe_get_shot() -> Option<Shot> {
	let res = rand::thread_rng().gen_range(12..27);

	println!("Generated: {}", res);

	match res {
	12..=16 => Some(Shot::Two),
	17..=21 => Some(Shot::Three),
	22..=26 => Some(Shot::FreeThrow),
	_ => None,
	}
}
