use std::borrow::Borrow;

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
struct Range {
    from: Vec2,
    to: Vec2,
}

#[derive(Debug, Clone)]
struct Probe {
    position: Vec2,
    velocity: Vec2,
    target: Range,
}

impl Probe {
    fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        self.velocity.x = i64::max(self.velocity.x - self.velocity.x.signum(), 0);
        self.velocity.y -= 1;

        // println!("Position: {},{} | Velocity: {},{}", self.position.x, self.position.y,
        //          self.velocity.x, self.velocity.y);
    }

    fn within_target(&self) -> bool {
        self.position.x >= self.target.from.x &&
            self.position.x <= self.target.to.x &&
            self.position.y >= self.target.from.y &&
            self.position.y <= self.target.to.y
    }

    fn simulate_for(&mut self, steps: usize) {
        for _ in 0..steps {
            self.update();
            if self.within_target() {
                return;
            }
            if self.position.y < self.target.to.y && self.velocity.x == 0 {
                // println!("Skipping drop");
                return;
            }
        }
    }
}

fn main() {
    let target = Range {
        from: Vec2 { x: 88, y: -157 },
        to: Vec2 { x: 125, y: -103 },
    };

    let min_vel_x = -1000;
    let max_vel_x = 1000;
    let min_vel_y = -1000;
    let max_vel_y = 1000;

    let mut count = 0;
    let mut highest: Option<Vec2> = None;
    for velX in min_vel_x..max_vel_x {
        for velY in min_vel_y..max_vel_y {
            let mut probe = Probe {
                position: Vec2 { x: 0, y: 0 },
                velocity: Vec2 { x: velX, y: velY },
                target: target.clone(),
            };
            probe.simulate_for(100000);

            if probe.within_target() {
                let score = velY * (velY + 1) / 2;

                if highest.is_some() {
                    let v = highest.unwrap();
                    let prev_score = v.y * (v.y + 1) / 2;
                    if score > prev_score {
                        highest = Some(Vec2 {x: velX, y: velY});
                    }
                } else {
                    highest = Some(Vec2 {x: velX, y: velY});
                }

                count += 1;
                println!("Velocity {},{} within target, score: {}", velX, velY, score);
            }
        }
    }

    let highest = highest.unwrap();
    let score = highest.y * (highest.y + 1) / 2;
    println!("Highest score for vel {},{}: {}", highest.x, highest.y, score);
    println!("Solutions: {}", count);
}

