pub fn tracker_player_scraper(user_id: &str){
    let tracker_player_url = "https://tracker.gg/csgo/profile/steam/";
    let combine_player_url = [tracker_player_url,user_id].join("");

    println!("{}", combine_player_url);

    let response = reqwest::blocking::get(
        combine_player_url
    ).unwrap().text().unwrap();

    println!("{}", response);
}