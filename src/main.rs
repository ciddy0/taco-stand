use macroquad::prelude::{camera::mouse, *};
use macroquad::rand::gen_range;

fn repair_shop(respawn_delay: &mut f64, money: &mut f64) {
    draw_rectangle(0.0, 0.0, 150.0, 100.0, WHITE);
    draw_text("Repair Shop: Tip rate increase+", 10.0, 10.0, 10.0, BLACK);
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        let rect_x = 0.0;
        let rect_y = 0.0;
        let rect_w = 150.0;
        let rect_h = 100.0;

        // Check if click is inside rectangle
        if mx >= rect_x
            && mx <= rect_x + rect_w
            && my >= rect_y
            && my <= rect_y + rect_h
            && *money >= 100.0
        {
            *respawn_delay = 0.5;
            *money = *money - 100.0;
            println!("Repair Shop clicked! Respawn delay changed to 0.5s")
        }
    }
}
fn add_stools(rate_of_money: &mut f64, money: &mut f64) {
    draw_rectangle(0.0, 150.0, 150.0, 100.0, WHITE);
    draw_text("Add Stools: Revenue increase+", 10.0, 170.0, 10.0, BLACK);
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        let rect_x = 0.0;
        let rect_y = 100.0;
        let rect_w = 150.0;
        let rect_h = 100.0;

        // Check if click is inside rectangle
        if mx >= rect_x
            && mx <= rect_x + rect_w
            && my >= rect_y
            && my <= rect_y + rect_h
            && *money >= 100.0
        {
            *rate_of_money = 0.02;
            *money = *money - 100.0;
            println!("Stools added! Revenue increased 2x!")
        }
    }
}
#[macroquad::main("Taco Stand")]
async fn main() {
    let mut money = 0.0;
    let mut rate_of_money = 0.01;
    let mut coin_pos = vec2(100.0, 100.0);
    let coin_radius = 20.0;
    let mut coin_visible = true;
    let mut last_spawn_time = get_time();
    let mut respawn_delay = 1.0;

    loop {
        money = money + rate_of_money;
        clear_background(RED);

        if !coin_visible && get_time() - last_spawn_time > respawn_delay {
            coin_pos = vec2(
                gen_range(50.0, screen_width() - 50.0),
                gen_range(100.0, screen_width() - 50.0),
            );
            coin_visible = true;
        }
        if coin_visible {
            draw_circle(coin_pos.x, coin_pos.y, coin_radius, BLACK);
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                let dx = mx - coin_pos.x;
                let dy = my - coin_pos.y;
                if dx * dx + dy * dy <= coin_radius * coin_radius {
                    money += 1.0;
                    coin_visible = false;
                    last_spawn_time = get_time();
                }
            }
        }

        repair_shop(&mut respawn_delay, &mut money);
        add_stools(&mut rate_of_money, &mut money);
        draw_text(&format!("Currency: ${:.0}", money), 20.0, 40.0, 30.0, BLACK);
        next_frame().await
    }
}
