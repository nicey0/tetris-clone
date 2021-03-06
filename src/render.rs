use piston_window::*;

use super::colpoint::ColPoint;
use super::conf::*;
use super::game::Game;
use super::util::*;

impl Game {
    pub fn draw_right(&self, c: &Context, g: &mut G2d) {
        rectangle(RIGHT, [WELLWIDTH, 0.0, RWIDTH, WINSIZE.1], c.transform, g);
    }

    pub fn draw_next(&self, c: &Context, g: &mut G2d) {
        // draw next piece background bit
        rectangle(
            NEXT,
            [WELLWIDTH + CELLSIZE, CELLSIZE, NWIDTH, NHEIGHT],
            c.transform,
            g,
        );
        // draw next piece
        for y in 0..4 {
            for x in 0..4 {
                if self.next.get_shape().contains(&(x, y)) {
                    rectangle(
                        self.next.get_color(),
                        [
                            WELLWIDTH + CELLSIZE + x as f64 * CELLSIZE + NWIDTH / 2.0
                                - (self.next.get_width() as f64 / 2.0 * CELLSIZE),
                            CELLSIZE + y as f64 * CELLSIZE + NHEIGHT / 2.0
                                - ((self.next.get_height()) as f64 / 2.0 * CELLSIZE),
                            CELLSIZE,
                            CELLSIZE,
                        ],
                        c.transform,
                        g,
                    );
                }
            }
        }
    }

    pub fn draw_text(
        &self,
        c: &Context,
        g: &mut G2d,
        text: &str,
        glyphs: &mut Glyphs,
        x: f64,
        y: f64,
    ) {
        Text::new_color([1.0; 4], (CELLSIZE as f64 * 0.75) as u32)
            .draw(
                text,
                glyphs,
                &c.draw_state,
                c.transform.trans(
                    (WELLWIDTH + CELLSIZE) + x,
                    (CELLSIZE * 2.0 + NHEIGHT + CELLSIZE / 2.0) + y,
                ),
                g,
            )
            .expect("error drawing text!");
    }

    pub fn draw_pieces(&self, c: &Context, g: &mut G2d) {
        let s = self.p.get_shape();
        for y in MAXY - BOARDY..MAXY {
            for x in 0..MAXX {
                if s.contains(&(x, y)) {
                    // draw block
                    rectangle(
                        self.p.get_color(),
                        [
                            x as f64 * CELLSIZE,
                            y as f64 * CELLSIZE - TOP_PAD,
                            CELLSIZE,
                            CELLSIZE,
                        ],
                        c.transform,
                        g,
                    );
                } else if self.pieces.contains(&ColPoint {
                    point: (x, y),
                    color: [0.0; 4],
                }) {
                    // draw block
                    rectangle(
                        self.pieces[self.pieces.iter().position(|e| e == &(x, y)).unwrap()].color,
                        [
                            x as f64 * CELLSIZE,
                            y as f64 * CELLSIZE - TOP_PAD,
                            CELLSIZE,
                            CELLSIZE,
                        ],
                        c.transform,
                        g,
                    );
                } else if self.shadow.shape.contains(&(x, y)) {
                    rectangle(
                        SHDOW,
                        [
                            x as f64 * CELLSIZE,
                            y as f64 * CELLSIZE - TOP_PAD,
                            CELLSIZE,
                            CELLSIZE,
                        ],
                        c.transform,
                        g,
                    );
                }
            }
        }
    }
}
