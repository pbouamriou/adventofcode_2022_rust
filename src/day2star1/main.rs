use std::io;

fn main() {
    use adventofcode::day2::Game;
    let mut lines = io::stdin().lines();
    let game = Game::parse_strategy_guide(&mut lines);
    println!(
        "Total game = {}, Total points = {}",
        game.number_of_matches, game.points
    );
}
