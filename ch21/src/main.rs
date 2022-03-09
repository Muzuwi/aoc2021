use std::ptr::replace;

#[derive(Debug)]
struct Player {
    position: usize,
    score: usize,
}

struct Game {
    players: [Player; 2],
    current_roll: usize,
    roll_count: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            players: [
                Player { position: 10 - 1, score: 0 },
                Player { position: 3 - 1, score: 0 }
            ],
            current_roll: 0,
            roll_count: 0,
        }
    }

    fn next_roll(&mut self) -> usize {
        let value = self.current_roll;
        self.current_roll = (self.current_roll + 1) % 100;
        self.roll_count += 1;
        value + 1
    }

    fn run(&mut self) {
        loop {
            for i in 0..2 {
                let result = self.next_roll() + self.next_roll() + self.next_roll();

                let player = &mut self.players[i];
                player.position = (player.position + result) % 10;
                player.score += (player.position) + 1;
                println!("Player {} score: {}", i, player.score);
                if player.score >= 1000 {
                    println!("Player {} won with {} points", i, player.score);

                    let losing_score = self.players[if i == 0 { 1 } else { 0 }].score;
                    println!("Losing player had {} points", losing_score);
                    println!("The dice was rolled {} times", self.roll_count);
                    println!("Final score: {}", losing_score * self.roll_count);
                    return;
                }
            }
        }
    }

    fn run_universe(&mut self) {
        loop {
            for i in 0..2 {
                let player = &mut self.players[i];

                player.position = (player.position + 1) % 10;
                player.score += player.position + 1;
                player.position = (player.position + 2) % 10;
                player.score += player.position + 1;
                player.position = (player.position + 3) % 10;
                player.score += player.position + 1;
            }
        }

    }
}


fn main() {
    let mut game = Game::new();
    game.run();
}
