use macroquad::prelude::*;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 25.;
struct Ship {
    pos: Vec2,
    rot: f32,
    vel: Vec2,
}

#[macroquad::main("Spacey")]
async fn main() {
    let mut ship = Ship {
        pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
        rot: 0.,
        vel: Vec2::new(0., 0.),
    };

    loop {
        clear_background(BLACK);

        let rotation = ship.rot.to_radians();
        let mut acc = -ship.vel / 80.;

        if is_key_down(KeyCode::Up) {
            acc = Vec2::new(rotation.sin(), -rotation.cos()) / 3.;
        }

        if is_key_down(KeyCode::Right) {
            ship.rot += 3.;
        } else if is_key_down(KeyCode::Left) {
            ship.rot -= 3.;
        }

        ship.vel += acc;
        if ship.vel.length() > 3. {
            ship.vel = ship.vel.normalize() * 3.;
        }
        ship.pos += ship.vel;

        let v1 = Vec2::new(
            ship.pos.x + rotation.sin() * SHIP_HEIGHT / 3.,
            ship.pos.y - rotation.cos() * SHIP_HEIGHT / 3.,
        );
        let v2 = Vec2::new(
            ship.pos.x - rotation.cos() * SHIP_BASE / 3. - rotation.sin() * SHIP_HEIGHT / 2.,
            ship.pos.y - rotation.sin() * SHIP_BASE / 3. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            ship.pos.x + rotation.cos() * SHIP_BASE / 3. - rotation.sin() * SHIP_HEIGHT / 2.,
            ship.pos.y + rotation.sin() * SHIP_BASE / 3. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        draw_triangle_lines(v1, v2, v3, 2., WHITE);

        next_frame().await
    }
}