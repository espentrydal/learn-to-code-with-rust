fn main() {
    let is_concert = true;
    let is_event = is_concert;
    dbg!(is_concert);
    dbg!(is_event);

    let sushi: String = String::from("Salmon");
    dbg!(&sushi);

    dbg!(eat_meal(sushi));

    let mut trip = start_trip();
    visit_philadelphia(&mut trip);
    visit_new_york(&mut trip);
    visit_boston(&mut trip);
    let itinerary = trip;
    show_itinerary(&itinerary);
}

fn eat_meal(mut meal: String) {
    meal.clear();
}

fn start_trip() -> String {
    String::new()
}

fn visit_philadelphia(trip: &mut String) {
    trip.push_str("Philadelphia");
    trip.push_str(", ");
}

fn visit_new_york(trip: &mut String) {
    trip.push_str("New York");
    trip.push_str(" and ");
}

fn visit_boston(trip: &mut String) {
    trip.push_str("Boston");
    trip.push('.');
}

fn show_itinerary(itinerary_string: &String) {
    println!("The plan is ... {}", itinerary_string);
}
