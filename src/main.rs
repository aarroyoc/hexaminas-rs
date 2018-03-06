extern crate piston_window;
extern crate graphics;
extern crate rand;
extern crate piston;

mod table;
mod mine;

use table::Table;
use piston_window::*;


fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hexaminas", [640, 480])
        .exit_on_esc(true).build().unwrap();
    let mut table = Table::new(15,15);
    let mut cursor = [0.0,0.0];
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, mut g| {
            clear([1.0; 4], g);
            table.draw(&c,&mut g);
        });
        if let Some(Button::Mouse(button)) = e.press_args() {
            if let Some(pos) = table.select(&cursor){
                if button == piston::input::MouseButton::Left {
                    println!("{:?}",pos);
                    let m = table.get_mines_around(pos);
                    println!("Minas alrededor: {}",m);

                }

                if button == piston::input::MouseButton::Right {

                }
            }
            
        }
        e.mouse_cursor(|x, y| {
            cursor = [x, y];
        });
    }
}
