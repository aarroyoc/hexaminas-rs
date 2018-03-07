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
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new("NotoSans-Regular.ttf", factory, TextureSettings::new()).unwrap();
    let mut gameover = false;
    let mut victory = false;
    while let Some(e) = window.next() {
        if !gameover{  
            window.draw_2d(&e, |c, mut g| {
                clear([1.0; 4], g);
                table.draw(&mut glyphs,&c,&mut g);
            });
            if let Some(Button::Mouse(button)) = e.press_args() {
                if let Some(pos) = table.select(&cursor){
                    if button == piston::input::MouseButton::Left {
                        let m = table.get_mines_around(pos);
                        if table.is_mine(pos){
                            gameover = true;
                        } else {
                            table.reveal(pos);
                            victory = table.victory();
                            gameover = victory;
                        }
                    }

                    if button == piston::input::MouseButton::Right {
                        table.flag(pos);
                    }
                }
                
            }
        } else if victory{
            window.draw_2d(&e, |c,g|{
                clear([0.0,0.0,0.0,1.0], g);
                text([1.0,1.0,1.0,1.0],24,"Victory",&mut glyphs,c.transform.trans(100.0,100.0),g).unwrap();
            });
        }else{
            window.draw_2d(&e, |c, g| {
                clear([0.0,0.0,0.0,1.0], g);
                text([1.0,1.0,1.0,1.0],24,"Game Over",&mut glyphs,c.transform.trans(100.0,100.0),g).unwrap();
            });
        }
        e.mouse_cursor(|x, y| {
            cursor = [x, y];
        });
    }
}
