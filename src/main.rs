use std::f32;

//mod hex;
use anyhow::Result;
use hexx::{Hex, HexLayout, HexOrientation, Vec2};
use macroquad::prelude::*;

#[macroquad::main("Hello-Macroquad")]
async fn main() -> Result<()> {
    let mut fullscreen = false;
    let mut hex_layout = HexLayout {
        scale: Vec2::new(48., 48.),
        orientation: HexOrientation::Pointy,
        origin: Vec2::new(screen_width() / 2., screen_height() / 2.),
    };
    let hex_texture = load_texture("assets/hex.png").await?;

    let origin = Hex::new(0, 0);
    let neighs = origin.all_neighbors();

    loop {
        // Input
        let (x, y) = mouse_position();
        let hovered_hex = hex_layout.world_pos_to_hex(Vec2::new(x, y));

        if is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Space) {
            hex_layout.orientation = match hex_layout.orientation {
                HexOrientation::Pointy => HexOrientation::Flat,
                HexOrientation::Flat => HexOrientation::Pointy,
            }
        }

        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            return Ok(());
        }

        if is_key_pressed(KeyCode::F) {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen);
        }

        // Changed window size is technicaly an input :D
        hex_layout.origin = Vec2::new(screen_width() / 2., screen_height() / 2.);

        // Draw
        clear_background(BLACK);

        for hex in neighs.iter() {
            let pos = hex_layout.hex_to_world_pos(*hex);
            draw_texture_ex(
                &hex_texture,
                pos.x - hex_layout.scale.x,
                pos.y - hex_layout.scale.y,
                WHITE,
                DrawTextureParams {
                    // Rotate if Orientation is Flat (Horizontal)
                    rotation: ((hex_layout.orientation == HexOrientation::Flat) as i8 as f32
                        * (f32::consts::PI / 2.)),
                    ..Default::default()
                },
            );
            draw_hexagon(
                pos.x,
                pos.y,
                hex_layout.scale.x - 1.,
                2.,
                hex_layout.orientation == HexOrientation::Pointy,
                GRAY,
                Color::from_rgba(0, 0, 0, 0),
            );
        }

        draw_text(
            &format!("Hello World! ({:4}, {:4}) -> {:?}", x, y, hovered_hex),
            12.,
            42.,
            32.,
            WHITE,
        );

        // Next frame
        next_frame().await;
    }
}
