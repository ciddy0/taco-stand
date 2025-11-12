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
    normal_texture: &Texture2D,
    hovered_texture: &Texture2D,
    purchased_texture: &Texture2D,
    is_purchased: bool,
) -> (bool, bool) {
    let (mx, my) = mouse_position();
    let hovered = mx >= x && mx <= x + w && my >= y && my <= y + h;

    // Determine which texture to use based on state
    let current_texture = if is_purchased {
        purchased_texture
    } else if hovered {
        hovered_texture
    } else {
        normal_texture
    };

    // Draw the button texture
    draw_texture_ex(
        current_texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(w, h)),
            ..Default::default()
        },
    );

    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left) && !is_purchased;
    (clicked, hovered)
}
#[macroquad::main(window_config)]
async fn main() {
    let mut money = 0.0;
    let mut rate_of_money = 0.01;
    let mut coin_pos = vec2(100.0, 100.0);
    let coin_radius = 32.0;
    let mut coin_visible = true;
    let mut last_spawn_time = get_time();
    let mut respawn_delay = 1.0;

    // Main background
    let background: Texture2D = load_texture("assets/background.png").await.unwrap();
    background.set_filter(FilterMode::Nearest);

    // Rounded rectangle background
    let rect_bg: Texture2D = load_texture("assets/background2.png").await.unwrap();
    rect_bg.set_filter(FilterMode::Nearest);

    // layer 1
    let layer1_bg: Texture2D = load_texture("assets/layer1.png").await.unwrap();
    layer1_bg.set_filter(FilterMode::Nearest);
    // layer 2
    let layer2_bg: Texture2D = load_texture("assets/layer2.png").await.unwrap();
    layer2_bg.set_filter(FilterMode::Nearest);

    // buttons
    let repair_taco_stand_button_normal: Texture2D =
        load_texture("assets/buttons/Repair_Taco_Stand_button.png")
            .await
            .unwrap();
    repair_taco_stand_button_normal.set_filter(FilterMode::Nearest);
    let repair_taco_stand_button_hovered: Texture2D =
        load_texture("assets/buttons/Repair_Taco_Stand_button_hovered.png")
            .await
            .unwrap();
    repair_taco_stand_button_hovered.set_filter(FilterMode::Nearest);
    let repair_taco_stand_button_purchased: Texture2D =
        load_texture("assets/buttons/Repair_Taco_Stand_button_purchased.png")
            .await
            .unwrap();
    repair_taco_stand_button_purchased.set_filter(FilterMode::Nearest);
    // Add this in the texture loading section (after repair_taco_stand buttons):

    // Button 2
    let add_stools_button_normal: Texture2D = load_texture("assets/buttons/add_stools_button.png")
        .await
        .unwrap();
    add_stools_button_normal.set_filter(FilterMode::Nearest);

    let add_stools_button_hovered: Texture2D =
        load_texture("assets/buttons/add_stools_button_hovered.png")
            .await
            .unwrap();
    add_stools_button_hovered.set_filter(FilterMode::Nearest);

    let add_stools_button_purchased: Texture2D =
        load_texture("assets/buttons/add_stools_button_purchased.png")
            .await
            .unwrap();
    add_stools_button_purchased.set_filter(FilterMode::Nearest);

    // Button 3
    let add_lights_button_normal: Texture2D = load_texture("assets/buttons/add_lights_button.png")
        .await
        .unwrap();
    add_lights_button_normal.set_filter(FilterMode::Nearest);

    let add_lights_button_hovered: Texture2D =
        load_texture("assets/buttons/add_lights_button_hovered.png")
            .await
            .unwrap();
    add_lights_button_hovered.set_filter(FilterMode::Nearest);

    let add_lights_button_purchased: Texture2D =
        load_texture("assets/buttons/add_lights_button_purchased.png")
            .await
            .unwrap();
    add_lights_button_purchased.set_filter(FilterMode::Nearest);

    // Button 4
    let alpastor_button_normal: Texture2D = load_texture("assets/buttons/alpastor_button.png")
        .await
        .unwrap();
    alpastor_button_normal.set_filter(FilterMode::Nearest);

    let alpastor_button_hovered: Texture2D =
        load_texture("assets/buttons/alpastor_button_hovered.png")
            .await
            .unwrap();
    alpastor_button_hovered.set_filter(FilterMode::Nearest);

    let alpastor_button_purchased: Texture2D =
        load_texture("assets/buttons/alpastor_button_purchased.png")
            .await
            .unwrap();
    alpastor_button_purchased.set_filter(FilterMode::Nearest);
    // load font
    let font: Font = load_ttf_font("fonts/MadimiOne-Regular.ttf").await.unwrap();

    // load cursor texture
    let cursor_texture: Texture2D = load_texture("assets/cursor.png").await.unwrap();
    cursor_texture.set_filter(FilterMode::Nearest);
    let pointer_texture: Texture2D = load_texture("assets/pointer.png").await.unwrap();
    pointer_texture.set_filter(FilterMode::Nearest);
    show_mouse(false);

    // load coin texture
    let coin_texture: Texture2D = load_texture("assets/coin.png").await.unwrap();
    coin_texture.set_filter(FilterMode::Nearest);

    // Rectangle properties
    let rect_w = 882.0;
    let rect_h = 542.0;
    let rect_x = (screen_width() - rect_w) / 2.0;
    let rect_y = (screen_height() - rect_h) / 2.0;

    let mut repair_shop_purchased = false;
    let mut add_stools_purchased = false;
    let mut add_lights_purchased = false;
    let mut alpastor_purchased = false;

    loop {
        println!("FPS: {}", get_fps());
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
        // draw layer 2
        draw_texture_ex(
            &layer2_bg,
            rect_x + 417.0,
            rect_y + 37.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(420.0, 467.0)),
                ..Default::default()
            },
        );
        // draw layer 1
        draw_texture_ex(
            &layer1_bg,
            rect_x + 417.0,
            rect_y + 37.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(420.0, 467.0)),
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
            &repair_taco_stand_button_normal,
            &repair_taco_stand_button_hovered,
            &repair_taco_stand_button_purchased,
            repair_shop_purchased,
        );
        if hovered {
            any_button_hovered = true;
        }
        if clicked {
            println!("Repair shop purchased!");
            rate_of_money = 0.02;
            money -= 100.0;
            repair_shop_purchased = true;
        }
        // Button 2
        let (clicked2, hovered2) = button(
            rect_x + 30.0,
            rect_y + 278.0, // Adjust Y position
            300.0,
            41.0,
            &add_stools_button_normal,
            &add_stools_button_hovered,
            &add_stools_button_purchased,
            add_stools_purchased,
        );
        if hovered2 {
            any_button_hovered = true;
        }
        if clicked2 && money >= 200.0 {
            println!("Button 2 purchased!");
            money -= 200.0;
            add_stools_purchased = true;
        }

        // Button 3
        let (clicked3, hovered3) = button(
            rect_x + 30.0,
            rect_y + 329.0,
            300.0,
            41.0,
            &add_lights_button_normal,
            &add_lights_button_hovered,
            &add_lights_button_purchased,
            add_lights_purchased,
        );
        if hovered3 {
            any_button_hovered = true;
        }
        if clicked3 && money >= 300.0 {
            println!("Button 3 purchased!");
            money -= 300.0;
            add_lights_purchased = true;
        }

        // Button 4
        let (clicked4, hovered4) = button(
            rect_x + 30.0,
            rect_y + 380.0,
            300.0,
            41.0,
            &alpastor_button_normal,
            &alpastor_button_hovered,
            &alpastor_button_purchased,
            alpastor_purchased,
        );
        if hovered4 {
            any_button_hovered = true;
        }
        if clicked4 && money >= 400.0 {
            println!("Button 4 purchased!");
            money -= 400.0;
            alpastor_purchased = true;
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
        if coin_visible {
            draw_texture_ex(
                &coin_texture,
                coin_pos.x - coin_radius / 2.0, // Center the texture on coin_pos
                coin_pos.y - coin_radius / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(coin_radius, coin_radius)),
                    ..Default::default()
                },
            );

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
