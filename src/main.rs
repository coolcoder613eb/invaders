#![windows_subsystem = "windows"]
#[cfg(feature = "audio")]
use macroquad::audio::{
    load_sound_from_bytes, play_sound, play_sound_once, stop_sound, PlaySoundParams,
};
use macroquad::prelude::*;
use macroquad::time::get_time;
use rand::{gen_range, ChooseRandom};

#[derive(Debug)]
struct Inputs {
    right: bool,
    left: bool,
    shoot: bool,
}

fn new_attackers(attackers: &mut Vec<Vec2>) {
    for row in 1..4 {
        for col in 1..11 {
            attackers.push(vec2((col * 60) as f32, (row * 60) as f32));
        }
    }
}

fn get_input() -> Inputs {
    let mut inputs = Inputs {
        right: false,
        left: false,
        shoot: false,
    };

    inputs.right = is_key_down(KeyCode::Right);
    inputs.left = is_key_down(KeyCode::Left);
    inputs.shoot = is_key_pressed(KeyCode::Space);
    #[cfg(feature = "touch")]
    for touch in touches() {
        if touch.position.x < screen_width() / 2.0 {
            inputs.left = true;
        } else if touch.position.x < screen_width() {
            inputs.right = true;
        }
        inputs.shoot = true;
    }
    inputs
}

#[macroquad::main("Space Invaders")]
async fn main() {
    let mut defender = vec2(screen_width() / 2.0, screen_height() - 30.0);
    let mut bullets: Vec<Vec2> = vec![];
    let mut fireballs: Vec<Vec2> = vec![];
    let mut attackers: Vec<Vec2> = vec![];
    let mut attacker_dir: bool = true;
    let mut stars: Vec<Vec2> = vec![];
    let mut score: usize = 0;
    let mut lives: isize = 3;
    let mut difficulty = 1.0;
    let mut last_shot_time = 0.0;
    let mut last_fireball_time = 0.0;

    #[cfg(feature = "audio")]
    #[cfg(feature = "wav")]
    let music = load_sound_from_bytes(include_bytes!("../assets/music.wav"))
        .await
        .expect("Failed to load music");
    #[cfg(feature = "audio")]
    #[cfg(not(feature = "wav"))]
    let music = load_sound_from_bytes(include_bytes!("../assets/music.ogg"))
        .await
        .expect("Failed to load music");

    #[cfg(feature = "audio")]
    #[cfg(feature = "wav")]
    let defender_shoot_sound =
        load_sound_from_bytes(include_bytes!("../assets/defender_shoot.wav"))
            .await
            .expect("Failed to load defender shoot sound");
    #[cfg(feature = "audio")]
    #[cfg(not(feature = "wav"))]
    let defender_shoot_sound =
        load_sound_from_bytes(include_bytes!("../assets/defender_shoot.ogg"))
            .await
            .expect("Failed to load defender shoot sound");

    #[cfg(feature = "audio")]
    #[cfg(feature = "wav")]
    let attacker_shoot_sound =
        load_sound_from_bytes(include_bytes!("../assets/attacker_shoot.wav"))
            .await
            .expect("Failed to load attacker shoot sound");
    #[cfg(feature = "audio")]
    #[cfg(not(feature = "wav"))]
    let attacker_shoot_sound =
        load_sound_from_bytes(include_bytes!("../assets/attacker_shoot.ogg"))
            .await
            .expect("Failed to load attacker shoot sound");

    #[cfg(feature = "audio")]
    #[cfg(feature = "wav")]
    let defender_explode_sound =
        load_sound_from_bytes(include_bytes!("../assets/defender_explode.wav"))
            .await
            .expect("Failed to load defender explode sound");
    #[cfg(feature = "audio")]
    #[cfg(not(feature = "wav"))]
    let defender_explode_sound =
        load_sound_from_bytes(include_bytes!("../assets/defender_explode.ogg"))
            .await
            .expect("Failed to load defender explode sound");

    #[cfg(feature = "audio")]
    #[cfg(feature = "wav")]
    let attacker_explode_sound =
        load_sound_from_bytes(include_bytes!("../assets/attacker_explode.wav"))
            .await
            .expect("Failed to load attacker explode sound");
    #[cfg(feature = "audio")]
    #[cfg(not(feature = "wav"))]
    let attacker_explode_sound =
        load_sound_from_bytes(include_bytes!("../assets/attacker_explode.ogg"))
            .await
            .expect("Failed to load attacker explode sound");

    // Define speeds in units per second
    const PROJECTILE_SPEED: f32 = 300.0; // pixels per second
    const DEFENDER_SPEED: f32 = 200.0; // pixels per second
    const SHOT_COOLDOWN: f64 = 0.3; // seconds between shots
    const FIREBALL_COOLDOWN: f64 = 0.5; // seconds between fireballs
    const ATTACKER_SPEED: f32 = 60.0; // base speed for attackers

    // set up random star positions
    for _star in 1..150 {
        stars.push(vec2(
            gen_range(0.0, screen_width()),
            gen_range(0.0, screen_height()),
        ));
    }

    // set up attackers initial positions
    new_attackers(&mut attackers);

    let mut last_frame_time = get_time();

    #[cfg(feature = "audio")]
    #[cfg(feature = "wav")]
    play_sound_once(
        &load_sound_from_bytes(include_bytes!("../assets/start.wav"))
            .await
            .expect("Failed to load start sound"),
    );
    #[cfg(feature = "audio")]
    #[cfg(not(feature = "wav"))]
    play_sound_once(
        &load_sound_from_bytes(include_bytes!("../assets/start.ogg"))
            .await
            .expect("Failed to load start sound"),
    );

    #[cfg(feature = "audio")]
    play_sound(
        &music,
        PlaySoundParams {
            looped: true,
            volume: 0.5,
        },
    );

    loop {
        let current_time = get_time();
        let delta_time = (current_time - last_frame_time) as f32;
        last_frame_time = current_time;

        if lives <= 0 {
            #[cfg(feature = "audio")]
            stop_sound(&music);
            #[cfg(feature = "audio")]
            #[cfg(feature = "wav")]
            play_sound_once(
                &load_sound_from_bytes(include_bytes!("../assets/game_over.wav"))
                    .await
                    .expect("Failed to load game over sound"),
            );
            #[cfg(feature = "audio")]
            #[cfg(not(feature = "wav"))]
            play_sound_once(
                &load_sound_from_bytes(include_bytes!("../assets/game_over.ogg"))
                    .await
                    .expect("Failed to load game over sound"),
            );
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

        let inputs = get_input();
        // handle keyboard input
        if inputs.right {
            if defender.x < (screen_width() - 25.0) {
                defender.x += DEFENDER_SPEED * delta_time;
            }
        }
        if inputs.left {
            if defender.x > 25.0 {
                defender.x -= DEFENDER_SPEED * delta_time;
            }
        }
        if inputs.shoot {
            if current_time - last_shot_time > SHOT_COOLDOWN {
                #[cfg(feature = "audio")]
                play_sound(
                    &defender_shoot_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.5,
                    },
                );
                bullets.push(defender);
                last_shot_time = current_time;
            }
        }

        if current_time - last_fireball_time > FIREBALL_COOLDOWN {
            if let Some(attacker) = attackers.choose() {
                #[cfg(feature = "audio")]
                play_sound(
                    &attacker_shoot_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.5,
                    },
                );
                fireballs.push(attacker.to_owned());
                last_fireball_time = current_time;
            }
        }

        clear_background(BLACK);
        // draw stars
        for star in &stars {
            draw_text("*", star.x, star.y, 10.0, WHITE);
        }

        // remove bullets that hit the top and move others forward
        bullets.retain_mut(|bullet| {
            if bullet.y > 10.0 + PROJECTILE_SPEED * delta_time {
                bullet.y -= PROJECTILE_SPEED * delta_time;
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

        //remove fireballs that hit defender and decrement the lives
        fireballs.retain_mut(|fireball| {
            if defender.distance(*fireball) < 30.0 {
                lives -= 1;
                #[cfg(feature = "audio")]
                play_sound_once(&defender_explode_sound);
                false
            } else {
                true
            }
        });

        // remove fireballs that hit the bottom and move others forward
        fireballs.retain_mut(|fireball| {
            if fireball.y < screen_height() - 10.0 {
                fireball.y += PROJECTILE_SPEED * delta_time;
                draw_triangle(
                    *fireball - vec2(2.0, 0.0),
                    *fireball + vec2(0.0, 10.0),
                    *fireball + vec2(2.0, 0.0),
                    GREEN,
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

        // First move attackers sideways
        for attacker in &mut attackers {
            let move_by = ATTACKER_SPEED * (1.0 + difficulty / 8.0) * delta_time;
            attacker.x += if attacker_dir { move_by } else { -move_by };
        }

        // Then check boundaries and update direction if needed
        let leftmost = attackers.iter().fold(f32::MAX, |acc, a| acc.min(a.x));
        let rightmost = attackers.iter().fold(f32::MIN, |acc, a| acc.max(a.x));

        if (leftmost < 30.0 && !attacker_dir) || (rightmost > screen_width() - 30.0 && attacker_dir)
        {
            attacker_dir = !attacker_dir;
            // Move all attackers down
            for attacker in &mut attackers {
                attacker.y += 25.0;
            }
        }

        // remove attacker and update score and difficulty when hit by bullet
        bullets.retain(|bullet| {
            let before_len = attackers.len();
            attackers.retain(|attacker| attacker.distance(*bullet) >= 25.0);
            let removed = before_len - attackers.len();
            score += removed;
            if removed > 0 {
                #[cfg(feature = "audio")]
                play_sound(
                    &attacker_explode_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.75,
                    },
                );
            }
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
                #[cfg(feature = "audio")]
                play_sound_once(&defender_explode_sound);
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
