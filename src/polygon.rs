use crate::framebuffer::Framebuffer;
use crate::line::line;
use raylib::prelude::*;

pub fn draw_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    if points.len() < 2 {
        return; // Not enough points to draw
    }

    for i in 0..points.len() {
        let start = points[i];
        let end = if i == points.len() - 1 {
            points[0] // Close the polygon
        } else {
            points[i + 1]
        };

        line(framebuffer, start, end);
    }
}

pub fn draw_filled_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    if points.len() < 3 {
        return;
    }

    // Find min and max Y (bounds of the polygon)
    let mut min_y = points[0].y;
    let mut max_y = points[0].y;

    for p in points.iter() {
        if p.y < min_y {
            min_y = p.y;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }

    // For every horizontal line between min_y and max_y
    let min_y = min_y as i32;
    let max_y = max_y as i32;

    for y in min_y..=max_y {
        let mut intersections: Vec<i32> = Vec::new();

        // For each edge of the polygon
        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = if i == points.len() - 1 {
                points[0]
            } else {
                points[i + 1]
            };

            let y1 = p1.y;
            let y2 = p2.y;

            // Ignore horizontal edges
            if y1 == y2 {
                continue;
            }

            // Check if this scanline intersects the edge
            if (y1 <= y as f32 && y2 > y as f32) || (y2 <= y as f32 && y1 > y as f32) {
                // Calculate the intersection X coordinate using linear interpolation
                let x = p1.x + (y as f32 - y1) * (p2.x - p1.x) / (y2 - y1);
                intersections.push(x as i32);
            }
        }

        // Sort the X intersections
        intersections.sort();

        // Fill pixels between pairs of intersections
        let mut i = 0;
        while i + 1 < intersections.len() {
            let start_x = intersections[i];
            let end_x = intersections[i + 1];

            for x in start_x..=end_x {
                framebuffer.set_pixel(x, y);
            }

            i += 2; // Move to next pair
        }
    }
}

pub fn draw_filled_polygon_with_holes(
    framebuffer: &mut Framebuffer,
    points: &[Vector2],
    hole_points: &[Vector2],
) {
    draw_filled_polygon(framebuffer, points);
    framebuffer.set_current_color(Color::BLACK);
    draw_filled_polygon(framebuffer, hole_points);
    framebuffer.set_current_color(Color::BLUEVIOLET);
}
