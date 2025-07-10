use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

pub fn line(framebuffer: &mut Framebuffer, start: Vector2, end: Vector2) {
    let x0 = start.x as i32;
    let y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    if dx > dy {
        let mut err = dx / 2;
        let mut y = y0;
        for i in 0..=dx {
            let x = x0 + i * sx;
            if x >= 0 && y >= 0 {
                framebuffer.set_pixel(x, y);
            }

            err -= dy;
            if err < 0 {
                y += sy;
                err += dx;
            }
        }
    } else {
        let mut err = dy / 2;
        let mut x = x0;
        for i in 0..=dy {
            let y = y0 + i * sy;
            if x >= 0 && y >= 0 {
                framebuffer.set_pixel(x, y);
            }

            err -= dx;
            if err < 0 {
                x += sx;
                err += dy;
            }
        }
    }
}
