use raylib::prelude::*;
use raylib::consts::KeyboardKey;
use raylib::color::Color;
use crate::ball::Ball;
use crate::paddle::Paddle;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 800;

pub fn single_player(rl: &mut RaylibHandle, thread: &RaylibThread, ball: &mut Ball, paddle1: &mut Paddle, paddle2: &mut Paddle, score1: &mut i32, score2: &mut i32, timer: &mut f32){
    let mut draw = rl.begin_drawing(thread);
    draw.clear_background(Color::BLACK);

    ball.change_dir(SCREEN_WIDTH, SCREEN_HEIGHT);
    paddle2.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_UP), draw.is_key_down(KeyboardKey::KEY_DOWN));

    //auto-controlling paddle1
    if ball.y < paddle1.y + 50{
        paddle1.speed = -5.0;
    }
    else if ball.y > paddle1.y + 50{
        paddle1.speed = 5.0;
    }
    else{
        paddle1.speed = 0.0;
    }
    paddle1.update(SCREEN_HEIGHT, false, false);

    update_game_state(ball, paddle1, paddle2, score1, score2);

    *timer -= draw.get_frame_time();

    draw_game(&mut draw, ball, paddle1, paddle2, *score1, *score2, *timer);
}

pub fn multi_player(rl: &mut RaylibHandle, thread: &RaylibThread, ball: &mut Ball, paddle1: &mut Paddle, paddle2: &mut Paddle, score1: &mut i32, score2: &mut i32, timer: &mut f32){
    let mut draw = rl.begin_drawing(thread);
    draw.clear_background(Color::BLACK);

    ball.change_dir(SCREEN_WIDTH, SCREEN_HEIGHT);
    paddle1.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_W), draw.is_key_down(KeyboardKey::KEY_S));
    paddle2.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_UP), draw.is_key_down(KeyboardKey::KEY_DOWN));

    update_game_state(ball, paddle1, paddle2, score1, score2);

    *timer -= draw.get_frame_time();

    draw_game(&mut draw, ball, paddle1, paddle2, *score1, *score2, *timer);
}

fn update_game_state(ball: &mut Ball, paddle1: &Paddle, paddle2: &Paddle, score1: &mut i32, score2: &mut i32){
    //Collision with left paddle
    if (ball.x <= paddle1.x + 10) && (ball.y >= paddle1.y) && (ball.y <= paddle2.y + 100){
        ball.vx = -ball.vx;
    }
    if (ball.x + 10 >= paddle2.x) && (ball.y >= paddle2.y) && (ball.y <= paddle2.y + 100){
        ball.vx = -ball.vx;
    }
    if (ball.x + 10) >= SCREEN_WIDTH{
        *score1 += 1;
        *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, -5.0, -5.0);
    }
    if ball.x <= 0{
        *score2 += 1;
        *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, 5.0, 5.0);
    }
}

fn draw_game(draw: &mut RaylibDrawHandle, ball: &mut Ball, paddle1: &Paddle, paddle2: &Paddle, score1: i32, score2: i32, timer: f32){
    draw.draw_text(&format!("Player 1: {}", score1), 10, 10, 30, Color::WHITE);
    draw.draw_text(&format!("Player 2: {}", score2), SCREEN_WIDTH - 150, 10, 30, Color::WHITE);
    draw.draw_text(&format!("Timer: {}", timer), SCREEN_WIDTH/2 - 50, 10, 30, Color::WHITE);
    draw.draw_text("Press 1 for Multiplayer, 2 for Single Player", 10, SCREEN_HEIGHT - 30, 20, Color::WHITE);

    paddle1.draw(draw);
    paddle2.draw(draw);
    ball.draw(draw);
}
