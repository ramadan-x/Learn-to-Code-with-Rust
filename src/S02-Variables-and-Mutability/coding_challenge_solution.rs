fn main() {
    let season: &str = "Autumn";

    let mut points_scored: i32 = 28;

    let team_name: &str = "Golden State Warriors";

    const TOUCHDOWN_POINTS: i32 = 6;

    let event_time: &str = "06:00";

    let event_time: i32 = 6;

    println!(
        "My favorite season is {}, my team is {}, \
         I scored {} points, and the event time is {}. \
         A touchdown is worth {} points.",
        season, team_name, points_scored, event_time, TOUCHDOWN_POINTS
    );

    let favorite_beverage: &str = "Coffee";

    #[allow(unused_variables)]
    let favorite_beverage: &str = "Coffee";
}
