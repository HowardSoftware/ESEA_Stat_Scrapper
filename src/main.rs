mod esea_scrapper;

fn main() {

    println!("Input the ESEA user you want to lookup: ");
    let mut user_id = String::new();

    std::io::stdin().read_line(&mut user_id).expect("failed to readline");

    esea_scrapper::esea_player_scraper(user_id.as_str());
}
