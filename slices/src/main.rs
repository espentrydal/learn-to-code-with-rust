fn main() {
    let mut cereals: [String; 5] = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let last_three = &mut cereals[2..];
    println!("{:?}", last_three);

    last_three[last_three.len() - 1] = String::from("Lucky Charms");
    println!("{:?}", last_three);

    let first_four = &cereals[..4];
    println!("{:?}", first_four);

    let mid_three = &cereals[1..4];
    println!("{:?}", mid_three);

    let cookie_crisp: &String = &cereals[0];
    let cookie = &cookie_crisp[..6];
    println!("{}", cookie);

    let puffs = &cereals[3][6..];
    println!("{}", puffs);
}
