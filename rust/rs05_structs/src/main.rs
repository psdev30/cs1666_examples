// Tuple struct
struct Point(i32, i32);

// Regular struct
struct Rectangle {
	top_left: Point,
	width: i32,
	height: i32,
	//         ^ last trailing comma optional
}

// Methods
impl Rectangle {
	// Note that there is no special constructor name
	// Do not have use "new"
	fn new(top_left: Point, width: i32, height: i32) -> Rectangle {
		Rectangle {
			top_left,
			width,
			height,
		}
	}

	fn identify(&self) {
		println!("I am a Rectangle");
	}
}

// Trait def
trait Shape {
	fn area(&self) -> i32;
	fn contains(&self, p: Point) -> bool;
}

//TODO: Provide an implementation of the Shape trait for Rectangle

//<Your code here>
impl Shape for Rectangle {
	fn area(&self) -> i32 {
		self.width * self.height
	}

	fn contains(&self, p: Point) -> bool {
		let x = self.top_left.0;
		let y = self.top_left.1;
		let x_max = x + self.width;
		let y_max = y + self.height;

		return (p.0 < x_max && p.0 > x) && (p.1 < y_max && p.1 > y)
	}
}

fn print_area<T: Shape> (some_shape: T) {
	println!("Shape's area: {}", some_shape.area());
}

fn main() {
	// Using typical computer graphics coordinates, (0, 0) in the top left
	// y increases as it goes down, x increases as it goes right
	let r = Rectangle::new(Point(5, 5), 10, 20);
	r.identify();

	print_area(r);

	let s = Rectangle::new(Point(0, 0), 10, 10);

	println!("s contains (5, 5): {}", s.contains(Point(5, 5)));
	println!("s contains (-1, -1): {}", s.contains(Point(-1, -1)));
	println!("s contains (7, 11): {}", s.contains(Point(7, 11)));
}
