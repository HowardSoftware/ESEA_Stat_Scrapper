mod tracker_scrapper;

fn main() {

    println!("Input the steam user you want to lookup: ");
    let mut user_id = String::new();

    std::io::stdin().read_line(&mut user_id).expect("failed to readline");

    tracker_scrapper::tracker_player_scraper(user_id.as_str());
}
