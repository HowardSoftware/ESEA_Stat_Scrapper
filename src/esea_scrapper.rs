pub fn esea_player_scraper(user_id: &str){
    let esea_player_url = "https://play.esea.net/users/";
    let combine_player_url = [esea_player_url,user_id].join("");

    println!("{}", combine_player_url);

    let response = reqwest::blocking::get(
        combine_player_url
    ).unwrap().text().unwrap();

    println!("{}", response);
}