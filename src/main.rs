use macroquad::prelude::*;
use rand::{gen_range, ChooseRandom};

fn new_attackers(attackers: &mut Vec<Vec2>) {
    for row in 1..4 {
        for col in 1..11 {
            attackers.push(vec2((col * 60) as f32, (row * 60) as f32));
        }
    }
}

#[macroquad::main("Space Invaders")]
async fn main() {
    let mut defender = vec2(screen_width() / 2.0, screen_height() - 30.0);
    let mut bullets: Vec<Vec2> = vec![];
    let mut fireballs: Vec<Vec2> = vec![];
    let mut attackers: Vec<Vec2> = vec![];
    let mut attacker_dir: bool = false;
    let mut stars: Vec<Vec2> = vec![];
    let mut score: usize = 0;
    let mut lives: isize = 3;
    let mut difficulty = 1.0;
    let mut time_since_space: u32 = 0;
    let mut time_since_fireball: u32 = 0;

    // set up random star positions
    for star in 1..150 {
        stars.push(vec2(
            gen_range(0.0, screen_width()),
            gen_range(0.0, screen_height()),
        ));
    }

    // set up attackers initial positions
    new_attackers(&mut attackers);

    loop {
        if lives <= 0 {
            loop {
                clear_background(RED);
                draw_text(
                    "G A M E  O V E R",
                    (screen_width() / 2.0) - 270.0,
                    screen_height() / 2.0,
                    80.0,
                    BLACK,
                );
                draw_text(
                    format!("Score: {}", score).as_str(),
                    (screen_width() / 2.0) - 140.0,
                    (screen_height() / 2.0) + 80.0,
                    80.0,
                    BLACK,
                );
                next_frame().await
            }
        }
        // handle keyboard input
        if is_key_down(KeyCode::Right) {
            if defender.x < (screen_width() - 25.0) {
                defender.x += 3.0;
            }
        }
        if is_key_down(KeyCode::Left) {
            if defender.x > 25.0 {
                defender.x -= 3.0;
            }
        }
        if is_key_pressed(KeyCode::Space) {
            if time_since_space > 20 {
                bullets.push(defender);
                time_since_space = 0;
            }
        } else {
            time_since_space += 1;
        }

        if time_since_fireball > 30 {
            // new fireball
            fireballs.push(
                attackers
                    .choose()
                    .expect("No attackers when pushing fireball!")
                    .to_owned(),
            );
            time_since_fireball = 0;
        } else {
            time_since_fireball += 1;
        }

        clear_background(BLACK);
        // draw stars
        for star in &stars {
            draw_text("*", star.x, star.y, 10.0, WHITE);
        }
        // remove bullets that hit the top and move others forward
        bullets.retain_mut(|bullet| {
            if bullet.y > 10.0 + 4.0 {
                bullet.y -= 4.0;
                draw_triangle(
                    *bullet - vec2(2.0, 0.0),
                    *bullet - vec2(0.0, 10.0),
                    *bullet + vec2(2.0, 0.0),
                    YELLOW,
                );
                true // keep the bullet
            } else {
                false // bullet is at the top, remove it
            }
        });

        fireballs.retain_mut(|fireball| {
            if defender.distance(*fireball) < 30.0 {
                lives -= 1;
                false
            } else {
                true
            }
        });

        // remove fireballs that hit the bottom and move others forward
        fireballs.retain_mut(|fireball| {
            if fireball.y < screen_height() - 10.0 {
                fireball.y += 4.0;
                draw_triangle(
                    *fireball - vec2(2.0, 0.0),
                    *fireball + vec2(0.0, 10.0),
                    *fireball + vec2(2.0, 0.0),
                    DARKPURPLE,
                );
                true // keep the fireball
            } else {
                false // fireball is at the top, remove it
            }
        });

        // draw defender
        draw_triangle(
            defender - vec2(25.0, 0.0),
            defender - vec2(0.0, 35.0),
            defender + vec2(25.0, 0.0),
            RED,
        );

        // move attackers
        // reverse direction if by edge, and move down
        let leftmost = attackers.iter().fold(f32::MAX, |acc, a| acc.min(a.x));
        let rightmost = attackers.iter().fold(f32::MIN, |acc, a| acc.max(a.x));

        if leftmost < 30.0 || rightmost > screen_width() - 30.0 {
            attacker_dir = !attacker_dir;
            for attacker in &mut attackers {
                attacker.y += 25.0;
            }
        }

        // move sideways
        for attacker in &mut attackers {
            let move_by = 1.0 + difficulty / 8.0;
            attacker.x += if attacker_dir { move_by } else { -move_by };
        }

        // remove attacker and update score and difficulty when hit by bullet
        bullets.retain(|bullet| {
            let before_len = attackers.len();
            attackers.retain(|attacker| attacker.distance(*bullet) >= 25.0);
            let removed = before_len - attackers.len();
            score += removed;
            difficulty += 0.1 * removed as f32;
            removed == 0 // keep bullet if it didn't hit anything (removed == 0), remove if it hit (removed > 0)
        });
        if attackers.len() == 0 {
            new_attackers(&mut attackers);
        }
        // draw attackers
        attackers.retain(|attacker| {
            if attacker.y > screen_height() {
                lives -= 1;
                false
            } else {
                draw_circle(attacker.x, attacker.y, 25.0, BLUE);
                true
            }
        });

        // draw top banner
        draw_text(
            format!("INVADERS | LIVES {} | SCORE {}", lives, score).as_str(),
            20.0,
            30.0,
            50.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
