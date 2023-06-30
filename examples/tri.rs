use sdl2::event::Event;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::pixels::PixelFormatEnum;
use glam::Vec3;
use ras::{Screen, Vertex};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Triangle test", 600, 600)
        .build()
        .map_err(|e| e.to_string())?;

    let mut rend = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let mut scr: Screen = Screen::new(600, 600);
    // let tri: [Vertex; 3] = [
    //     Vertex::new(Vec3::new(0., 0., 5.)),
    //     Vertex::new(Vec3::new(-2., 1.5, 5.)),
    //     Vertex::new(Vec3::new(0.5, 1., 5.))
    // ];

    let tris: Vec<[Vertex; 3]> = (0..5000).map(|_| [
        Vertex::new(Vec3::new(0., 0., 5.)),
        Vertex::new(Vec3::new(-2., 1.5, 5.)),
        Vertex::new(Vec3::new(0.5, 1., 5.))
    ]).collect();

    // let projected: [Vertex; 3] = tri.map(|v| ras::project_vert(v, 600, 600));

    let texture_creator = rend.texture_creator();
    let mut scrtex: Texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 600, 600)
        .map_err(|e| e.to_string())?;

    let timer_subsystem = sdl_context.timer().unwrap();
    let mut start_ticks = timer_subsystem.ticks();

    'running: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit {..} => break 'running,
                _ => ()
            }
        }

        for tri in &tris {
            ras::tri(tri, &mut scr);
        }

        // Render
        rend.set_draw_color(Color::RGB(0, 0, 0));
        rend.clear();

        // Render filled
        scrtex.with_lock(None, |buf: &mut [u8], pitch: usize| {
            let color_slice = scr.color.as_mut_slice();
            for y in 0..600 {
                for x in 0..600 {
                    let offset: usize = y * pitch + x * 3;
                    buf[offset] = (color_slice[y * 600 + x].x * 255.) as u8;
                    buf[offset + 1] = (color_slice[y * 600 + x].y * 255.) as u8;
                    buf[offset + 2] = (color_slice[y * 600 + x].z * 255.) as u8;
                }
            }
        })?;

        rend.copy_ex(
            &scrtex,
            None,
            Rect::new(0, 0, 600, 600),
            0.,
            None,
            false,
            false
        )?;

        // Render wireframe
        // rend.set_draw_color(Color::RGB(255, 0, 0));
        // rend.draw_line(
        //     Point::new(projected[0].pos.x as i32, projected[0].pos.y as i32),
        //     Point::new(projected[1].pos.x as i32, projected[1].pos.y as i32)
        // )?;

        // rend.draw_line(
        //     Point::new(projected[2].pos.x as i32, projected[2].pos.y as i32),
        //     Point::new(projected[1].pos.x as i32, projected[1].pos.y as i32)
        // )?;

        // rend.draw_line(
        //     Point::new(projected[0].pos.x as i32, projected[0].pos.y as i32),
        //     Point::new(projected[2].pos.x as i32, projected[2].pos.y as i32)
        // )?;

        rend.present();

        let curr = timer_subsystem.ticks();
        println!("Elapsed: {} ms", curr - start_ticks);
        start_ticks = curr;
    }

    Ok(())
}

