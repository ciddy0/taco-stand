use macroquad::prelude::{camera::mouse, *};
use macroquad::rand::gen_range;

fn window_config() -> Conf {
    Conf {
        window_title: "Taco Stand".to_string(),
        fullscreen: false,
        window_width: 1024,
        window_height: 768,
        window_resizable: false,
        ..Default::default()
    }
}
fn button(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    text: &str,
    bg: &Texture2D,
    inner_bg: &Texture2D,
    bg_hovered: &Texture2D,
    font: &Font,
) -> (bool, bool) {
    let (mx, my) = mouse_position();
    let hovered = mx >= x && mx <= x + w && my >= y && my <= y + h;

    let current_bg = if hovered { bg_hovered } else { inner_bg };
    draw_texture_ex(
        &bg,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(w, h)),
            ..Default::default()
        },
    );

    draw_texture_ex(
        &current_bg,
        x + 4.0,
        y + 4.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(185.0, 34.0)),
            ..Default::default()
        },
    );
    let text_size = 12.0;
    let text_x = x + 4.0 + 8.0;
    let text_y = y + 4.0 + 15.0;
    draw_text_ex(
        text,
        text_x,
        text_y,
        TextParams {
            font: Some(font),
            font_size: text_size as u16,
            color: BLACK,
            ..Default::default()
        },
    );

    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);
    (clicked, hovered)
}
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
#[macroquad::main(window_config)]
async fn main() {
    let mut money = 0.0;
    let mut rate_of_money = 0.01;
    let mut coin_pos = vec2(100.0, 100.0);
    let coin_radius = 20.0;
    let mut coin_visible = true;
    let mut last_spawn_time = get_time();
    let mut respawn_delay = 1.0;

    // Main background
    let background: Texture2D = load_texture("assets/background.png").await.unwrap();
    background.set_filter(FilterMode::Nearest);

    // Rounded rectangle background
    let rect_bg: Texture2D = load_texture("assets/background2.png").await.unwrap();
    rect_bg.set_filter(FilterMode::Nearest);

    // Rounded button background
    let button_bg: Texture2D = load_texture("assets/button_background.png").await.unwrap();
    button_bg.set_filter(FilterMode::Nearest);

    // Rounded inner button background
    let inner_button_bg: Texture2D = load_texture("assets/inner_button_background.png")
        .await
        .unwrap();
    inner_button_bg.set_filter(FilterMode::Nearest);

    let inner_button_bg_hovered: Texture2D =
        load_texture("assets/inner_button_background_hovered.png")
            .await
            .unwrap();
    inner_button_bg_hovered.set_filter(FilterMode::Nearest);

    // load font
    let font: Font = load_ttf_font("fonts/MadimiOne-Regular.ttf").await.unwrap();

    // load cursor texture
    let cursor_texture: Texture2D = load_texture("assets/cursor.png").await.unwrap();
    cursor_texture.set_filter(FilterMode::Nearest);
    let pointer_texture: Texture2D = load_texture("assets/pointer.png").await.unwrap();
    pointer_texture.set_filter(FilterMode::Nearest);
    show_mouse(false);

    // Rectangle properties
    let rect_w = 882.0;
    let rect_h = 542.0;
    let rect_x = (screen_width() - rect_w) / 2.0;
    let rect_y = (screen_height() - rect_h) / 2.0;

    loop {
        // Update currency
        money += rate_of_money;

        // Track if ANY button is hovered
        let mut any_button_hovered = false;

        // Draw main background (fullscreen)
        draw_texture_ex(
            &background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw the rounded rectangle with background2
        draw_texture_ex(
            &rect_bg,
            rect_x,
            rect_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(rect_w, rect_h)),
                ..Default::default()
            },
        );

        // currency text
        let text_size = 12.0;
        let text_x = rect_x + 8.0;
        let text_y = rect_y + 490.0;
        draw_text_ex(
            &format!("Currency: ${:.0}", money),
            text_x,
            text_y,
            TextParams {
                font: Some(&font),
                font_size: text_size as u16,
                color: WHITE,
                ..Default::default()
            },
        );

        let (clicked, hovered) = button(
            rect_x + 30.0,
            rect_y + 227.0,
            300.0,
            41.0,
            "Repair Shop",
            &button_bg,
            &inner_button_bg,
            &inner_button_bg_hovered,
            &font,
        );
        if clicked {
            println!("Rate of money upgraded!");
            rate_of_money = 0.02;
        }

        if hovered {
            any_button_hovered = true;
        }

        // Coin spawn logic
        if !coin_visible && get_time() - last_spawn_time > respawn_delay {
            coin_pos = vec2(
                gen_range(50.0, screen_width() - 50.0),
                gen_range(100.0, screen_height() - 50.0),
            );
            coin_visible = true;
        }

        // Draw coin
        // if coin_visible {
        //     draw_circle(coin_pos.x, coin_pos.y, coin_radius, BLACK);
        //     if is_mouse_button_pressed(MouseButton::Left) {
        //         let (mx, my) = mouse_position();
        //         let dx = mx - coin_pos.x;
        //         let dy = my - coin_pos.y;
        //         if dx * dx + dy * dy <= coin_radius * coin_radius {
        //             money += 1.0;
        //             coin_visible = false;
        //             last_spawn_time = get_time();
        //         }
        //     }
        // }

        // Buttons
        // repair_shop(&mut respawn_delay, &mut money);
        // add_stools(&mut rate_of_money, &mut money);

        // Currency text
        // draw_text(&format!("Currency: ${:.0}", money), 20.0, 40.0, 30.0, BLACK);
        // draw custom mouse
        // Determine cursor based on any button hover
        let cursor = if any_button_hovered {
            &pointer_texture
        } else {
            &cursor_texture
        };
        let (mx, my) = mouse_position();
        draw_texture_ex(
            cursor,
            mx,
            my,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
        next_frame().await;
    }
}
