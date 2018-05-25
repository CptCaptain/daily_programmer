// https://www.reddit.com/r/dailyprogrammer/comments/8jcffg/20180514_challenge_361_easy_tally_program/

fn main() {
    println!("Hello, world!");
    let input = "EbAAdbBEaBaaBBdAccbeebaec";
    let player_list = String::from(init_player_list(input.to_string()));
    println!("{}", player_list);

    let tally = tally(player_list, input.to_string());
    print!("{:?}", tally);
}

fn init_player_list(input: String) -> String{
    let mut result = String::new();
    let ch = input.chars();

    for cha in ch{
        let lch = cha.to_lowercase().next().unwrap();
        if !result.contains(lch){
            result.push(lch);
        }
    }
    return result
}

fn tally(player_list: String, tally: String)  -> Vec<(char, i32)>{
    let mut v: Vec<(char, i32)> = Vec::new();
    for (_i, player) in player_list.chars().enumerate() {
        let count = (tally.matches(player).count() as i32) - (tally.matches(player
            .to_uppercase().next().unwrap()).count() as i32);
        v.push((player, count));
    }
    v.sort_by_key(|v| v.1);
    v.reverse();
    return v
}
