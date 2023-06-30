use macroquad::color::*;
use macroquad::window::*;
use macroquad::shapes::*;
use glam::Vec3;
use ras::{Screen, Vertex};

#[macroquad::main(window_conf)]
async fn main() {
    let mut scr: Screen = Screen::new(600, 600);
    let tri: [Vertex; 3] = [
        Vertex::new(Vec3::new(0., 0., 5.)),
        Vertex::new(Vec3::new(1., -0.2, 5.)),
        Vertex::new(Vec3::new(0.5, 1., 5.))
    ];

    let projected: [Vertex; 3] = tri.map(|v| ras::project_vert(v, 600, 600));

    loop {
        ras::tri(&tri, &mut scr);

        clear_background(BLACK);

        for y in 0..scr.h {
            for x in 0..scr.w {
                let c: Vec3 = scr.color[y * scr.w + x];
                draw_rectangle(x as f32, y as f32, 1., 1.,
                    Color::from_rgba(
                        (c.x * 255.) as u8,
                        (c.y * 255.) as u8,
                        (c.z * 255.) as u8,
                        255
                    )
                );
            }
        }

        draw_line(projected[0].pos.x, projected[0].pos.y, projected[1].pos.x, projected[1].pos.y, 2., RED);
        draw_line(projected[2].pos.x, projected[2].pos.y, projected[1].pos.x, projected[1].pos.y, 2., RED);
        draw_line(projected[0].pos.x, projected[0].pos.y, projected[2].pos.x, projected[2].pos.y, 2., RED);

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_resizable: false,
        window_width: 600,
        window_height: 600,
        window_title: String::from("Triangle test"),
        ..Default::default()
    }
}

