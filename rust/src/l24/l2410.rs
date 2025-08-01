pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
    let mut players = players;
    let mut trainers = trainers;

    players.sort();
    trainers.sort();

    let mut result = 0;
    let mut curr_player = 0;
    let mut curr_trainer = 0;

    loop {
        if curr_player >= players.len() || curr_trainer >= trainers.len() {
            break;
        }
        if players[curr_player] <= trainers[curr_trainer] {
            curr_player += 1;
            curr_trainer += 1;
            result += 1;
            continue;
        } else {
            curr_trainer += 1;
            continue;
        }
    }

    result
}
