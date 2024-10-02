use notan::draw::*;
use notan::math::Vec2;
use notan::prelude::*;

mod player;
mod tilemap;
mod vertices;

use player::Player;
use tilemap::{SlopeType, TileMap, TileType, TILE_SIZE};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[derive(AppState)]
struct State {
    player: Player,
    tilemap: TileMap,
    font: Font,
    fps: f32,
}

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::new()
        .set_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .set_title("Cave Story Movement");

    notan::init_with(setup)
        .add_config(win)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn setup(_app: &mut App, gfx: &mut Graphics) -> State {
    let mut player = Player::new(50.0, 80.0, 24., 24.);
    let mut tilemap = TileMap::new(20, 20);

    // Set up some tiles for testing
    for x in 0..20 {
        tilemap.set_tile(x, 14, TileType::Solid); // Ground
        tilemap.set_tile(x, 15, TileType::Solid); // Ground
        tilemap.set_tile(x, 16, TileType::Solid); // Ground
    }
    tilemap.set_tile(10, 13, TileType::Solid); // Wall
                                               // Create a big up ramp
    for x in 5..10 {
        tilemap.set_tile(x, 14 - (x - 5), TileType::Slope(SlopeType::LeftUp));
    }

    // Create a big down ramp
    for x in 10..15 {
        tilemap.set_tile(x, 10 + (x - 10), TileType::Slope(SlopeType::RightUp));
    }

    let font = gfx
        .create_font(include_bytes!("../assets/Ubuntu-B.ttf"))
        .unwrap();

    let fps = _app.timer.fps();

    State {
        player,
        tilemap,
        font,
        fps,
    }
}

fn update(app: &mut App, state: &mut State) {
    let dt = app.timer.delta_f32();

    // Handle input
    let move_left = app.keyboard.is_down(KeyCode::Left);
    let move_right = app.keyboard.is_down(KeyCode::Right);
    let jump_pressed = app.keyboard.was_pressed(KeyCode::Space);
    let jump_released = app.keyboard.was_released(KeyCode::Space);

    // Update player
    state.player.set_movement(move_left, move_right);
    if jump_pressed {
        state.player.jump();
    }
    state.player.update(&state.tilemap.collision_mask(), dt);
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw_tilemap(&mut draw, &state.tilemap);
    &state.player.render(&mut draw);

    // Debug rendering
    state.player.debug_render(&mut draw, &state.font);
    state.tilemap.debug_render(&mut draw, &state.font);

    // Draw FPS and player position
    draw.text(&state.font, &format!("FPS: {}", &state.fps))
        .position(10.0, 20.0)
        .size(20.0)
        .color(Color::WHITE);

    draw.text(
        &state.font,
        &format!(
            "Pos: ({:.2}, {:.2})",
            state.player.polygon.position.0, state.player.polygon.position.1
        ),
    )
    .position(10.0, 40.0)
    .size(20.0)
    .color(Color::WHITE);

    gfx.render(&draw);
}

fn draw_tilemap(draw: &mut Draw, tilemap: &TileMap) {
    for y in 0..tilemap.height {
        for x in 0..tilemap.width {
            match tilemap.get_tile(x, y) {
                TileType::Solid => {
                    draw.rect(
                        (x as f32 * TILE_SIZE, y as f32 * TILE_SIZE),
                        (TILE_SIZE, TILE_SIZE),
                    )
                    .color(Color::GRAY);
                }
                TileType::Slope(slope_type) => {
                    let (x, y) = (x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
                    match slope_type {
                        SlopeType::LeftUp => {
                            draw.triangle(
                                (x, y + TILE_SIZE),
                                (x + TILE_SIZE, y + TILE_SIZE),
                                (x + TILE_SIZE, y),
                            )
                            .color(Color::GRAY);
                        }
                        SlopeType::RightUp => {
                            draw.triangle(
                                (x, y + TILE_SIZE),
                                (x + TILE_SIZE, y + TILE_SIZE),
                                (x, y),
                            )
                            .color(Color::GRAY);
                        }
                    }
                }
                TileType::Empty => {}
            }
        }
    }
}
