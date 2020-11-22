extern crate piston_window;

use piston_window::*;
use std::collections::{HashMap};
use std::borrow::Borrow;
use collider::{Collider, HbEvent, HbId, HbProfile};
use collider::geom::{Shape, v2, Vec2};
use std::any::Any;

#[derive(Copy, Clone, Debug)]
struct DemoHbProfile { id: HbId } // add any additional identfying data to this struct

impl HbProfile for DemoHbProfile {
    fn id(&self) -> HbId { self.id }
    fn can_interact(&self, _other: &DemoHbProfile) -> bool { true }
    fn cell_width() -> f64 { 4.0 }
    fn padding() -> f64 { 0.01 }
}

fn main() {
    let mut pressed_keys = HashMap::new();
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true).build().unwrap();

    let mut x = 20.0;
    let mut y = 0.0;
    let mut x2 = 0.0;
    let mut y2 = 240.0;

    let mut collider: Collider<DemoHbProfile> = Collider::new();
    let hitbox = Shape::square(10.0).place(v2(x, y)).moving(v2(0.0, 0.000001));
    let overlaps = collider.add_hitbox(DemoHbProfile { id: 0 }, hitbox);
    let hitbox2 = Shape::rect(Vec2::new(500.0, 10.0)).place(v2(x2, y2)).moving(v2(0.0, 0.0));
    let overlaps2 = collider.add_hitbox(DemoHbProfile { id: 1 }, hitbox2);
    let mut last_time= 0.0;

    while let Some(event) = window.next() {
        if let Some(btn_args) = event.button_args() {
            match btn_args.button {
                Button::Keyboard(key) => {
                    match btn_args.state {
                        ButtonState::Press => { pressed_keys.insert(key, true); }
                        ButtonState::Release => { pressed_keys.remove(&key); }
                    }
                }
                _ => {}
            }
        }
        for key in pressed_keys.borrow() {
            match key.0 {
                Key::Left => { x -= 1.25; }
                Key::Right => { x += 1.25; }
                Key::Up => { y -= 1.0; }
                Key::Down => { y += 1.0; }
                _ => {}
            }
        }
        {
            let time = collider.next_time();
            collider.set_time(last_time + 0.000000000000000000000001);
            last_time = time;
            if let Some((event, profile_1, profile_2)) = collider.next() {
                println!("{} / {}", profile_1.id, profile_2.id)
            }
            x = collider.get_hitbox(0).value.pos.x;
            y = collider.get_hitbox(0).value.pos.y;
            x2 = collider.get_hitbox(1).value.pos.x;
            y2 = collider.get_hitbox(1).value.pos.y;
        }
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0],
                      [x, y, 10.0, 10.0],
                      context.transform,
                      graphics);
            rectangle([1.0, 1.0, 0.0, 1.0],
                      [x2, y2, 500.0, 10.0],
                      context.transform,
                      graphics);
        });
    }
}
