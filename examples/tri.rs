use ras::{Screen, Vertex};
use glam::{Vec2, Vec3};
use macroquad::{
    window::*,
    color::*,
    input::*,
    time::*,
    texture::*
};
use image::DynamicImage;

#[macroquad::main(window_conf)]
async fn main() {
    let mut scr: Screen = Screen::new(600, 600);

    let mut tris: Vec<[Vertex; 3]> = (0..100).map(|_| [
        Vertex::new(Vec3::new(0., 0., 3.), Vec2::new(0., 0.)),
        Vertex::new(Vec3::new(1., 0., 3.), Vec2::new(1., 0.)),
        Vertex::new(Vec3::new(1., 1., 3.), Vec2::new(1., 1.))
    ]).collect();

    let image: DynamicImage = image::open("res/test.png")
                                .map_err(|e| e.to_string()).unwrap();
    let mut bytes: Vec<u8> = vec![0; 600 * 600 * 4];

    loop {
        if is_key_pressed(KeyCode::Space) {
            println!("Fps {}", get_fps());
        }

        // Fill screen buffer
        scr.clear();
        for tri in &mut tris {
            ras::tri(tri, &image, &mut scr);
        }

        // Render
        clear_background(BLACK);

        let bytes_slice: &mut [u8] = bytes.as_mut_slice();
        for i in 0..(600 * 600) {
            let offset: usize = i * 4;
            bytes_slice[offset] = (scr.color[i].x * 255.) as u8;
            bytes_slice[offset + 1] = (scr.color[i].y * 255.) as u8;
            bytes_slice[offset + 2] = (scr.color[i].z * 255.) as u8;
            bytes_slice[offset + 3] = 255;
        }

        draw_texture(Texture2D::from_rgba8(600, 600, bytes.as_slice()), 0., 0., WHITE);

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Raycast demo"),
        window_width: 600,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

