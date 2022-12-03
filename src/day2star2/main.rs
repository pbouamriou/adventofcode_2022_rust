use std::io;

fn main() {
    use adventofcode::day2::Game;
    let mut lines = io::stdin().lines();
    let game = Game::parse_ultra_top_secret_strategy_guide(&mut lines);
    println!(
        "Total game (with ultra top secret guide) = {}, Total points = {}",
        game.number_of_matches, game.points
    );
}
