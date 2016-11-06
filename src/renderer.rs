use piston_window::*;

use game::GameState;
use vec2::Vec2;
use map::CELL_SIZE;

use std::sync::{Arc, Mutex};
use std::thread;

pub struct Renderer {
    game: Arc<Mutex<Option<GameState>>>
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Result<Renderer, String> {
        let game = Arc::new(Mutex::new(None));
        let game_ref = game.clone();
        thread::spawn(move || {
            let mut window: PistonWindow = WindowSettings::new("RTS Renderer", (width, height))
                                                          .exit_on_esc(true)
                                                          .build()
                                                          .unwrap();
            
            while let Some(e) = window.next() {
                window.draw_2d(&e, |c, g| {
                    let guard = game.lock().unwrap();
                    if let Some(ref game) = *guard {
                        render_onto(game, c, g);
                    }
                });
            }
        });
        Ok(Renderer {
            game: game_ref
        })
    }

    pub fn render(&mut self, game: &GameState) {
        let mut guard = self.game.lock().unwrap();
        *guard = Some(game.clone());
    }
}

fn render_onto(game: &GameState, mut c: Context, g: &mut G2d) {
    clear([0., 0., 0., 1.], g);
    let Vec2(mapw, maph) = game.map.size().to_vec();
    if let Some(vp) = c.viewport {
        let (vpw, vph) = (vp.rect[2] as f64, vp.rect[3] as f64);
        let (tw, th) = (vpw / mapw, vph / maph);
        if tw > th {
            c = c.scale(th, th);
        }
        else {
            c = c.scale(tw, tw);
        }
    }
    for loc in game.map.loc_iter() {
        let t = game.map[loc];
        let col = if game.map.tile_map.tile_info(t).unwrap().solid {
            [0.7, 0.7, 0.7, 1.0]
        }
        else {
            [0.7, 0.1, 0.0, 1.0]
        };
        let tl = loc.top_left();
        rectangle(col, [tl.0, tl.1, CELL_SIZE, CELL_SIZE], c.transform, g);
    }
    for s in game.structures_iter() {
        let tl = s.loc.top_left() + Vec2(CELL_SIZE / 4., CELL_SIZE / 4.);
        let size = s.kind.size().to_vec() - Vec2(CELL_SIZE / 8., CELL_SIZE / 8.);
        rectangle([0.6, 1., 1., 1.], [tl.0, tl.1, size.0, size.1], c.transform, g);
    }
    for u in game.units_iter() {
        let radius = u.kind.radius();
        let Vec2(x, y) = u.pos;
        let c = c.trans(x, y);
        ellipse([1., 0.6, 1., 1.], [-radius, -radius, radius, radius], c.transform, g);
    }
}
