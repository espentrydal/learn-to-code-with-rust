use rug::Integer;

fn main() {
   dbg!(color_to_number("red"));
   dbg!(color_to_number("green"));
   dbg!(color_to_number("blue"));
   dbg!(factorial(1));
   dbg!(factorial(2));
   dbg!(factorial(3));
   dbg!(factorial(4));
   dbg!(factorial(9000));
   dbg!(factorial_rec(1));
   dbg!(factorial_rec(2));
   dbg!(factorial_rec(3));
   dbg!(factorial_rec(4));
   dbg!(factorial_rec(9000));
   // dbg!(factorial(-2));
}

fn color_to_number(color: &str) -> i32 {
    match color {
	"red" => 1,
	"green" => 2,
	"blue" => 3,
	_ => 0,
    }
}


fn factorial(number: i32) -> Integer {
    if number <= 0 {
	panic!("Provide a positive integer");
    }
    let mut product = Integer::from(1);
    for n in 1..=number {
	product *= n;
    }
    product
}

fn factorial_rec(number: i32) -> Integer {
    if number <= 0 {
	panic!("Provide a positive integer");
    }
    if number == 1 {
	Integer::from(1)
    }
    else { Integer::from(number) * factorial(number - 1)}
}
