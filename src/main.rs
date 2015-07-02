#![feature(plugin)]
#![plugin(gentest)]

extern crate sdl2;
extern crate lierosim;

use sdl2::video::{WindowPos, Window, OPENGL};
use sdl2::event::{Event};
use sdl2::rect::{Rect};
//use sdl2::surface::{Surface};
use sdl2::render::{Renderer, RenderDriverIndex};
use sdl2::pixels::{Color, PixelFormatEnum};
//use sdl2::timer::{delay};

fn main() {

    let ctx = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let window = Window::new("Liero", WindowPos::PosCentered, WindowPos::PosCentered, 640, 400, OPENGL)
        .unwrap();
        
    window.show();

    //let screen = window.get_surface().unwrap();
    let renderer = Renderer::from_window(window, RenderDriverIndex::Auto, sdl2::render::ACCELERATED | sdl2::render::PRESENTVSYNC).unwrap();
    let mut tex = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (512, 512)).unwrap();



    let mut drawer = renderer.drawer();
   
    let mut events = ctx.event_pump();

    let mut phase = 0;

    'eventloop: loop {
        for ev in events.poll_iter() {
            match ev {
                Event::Quit {..} => break 'eventloop,
                _ => {}
            }
        }

        //screen.fill_rect(Rect::new(5, 5, 630, 470));
        //window.update_surface();
        drawer.set_draw_color(Color::RGB(0, 0, 0));
        drawer.clear();

        tex.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in (0..200) {
                for x in (0..320) {
                    let offset = y*pitch + x*3;
                    buffer[offset + 0] = (x + phase) as u8;
                    buffer[offset + 1] = (y + phase) as u8;
                    buffer[offset + 2] = 0;
                }
            }
        }).unwrap();


        //drawer.set_draw_color(Color::RGB(40, 40, 40));
        //drawer.fill_rect(Rect::new(5, 5, 630, 470));
        drawer.copy(&tex, Some(Rect::new(0, 0, 320, 200)), None);
        drawer.present();

        //delay(1000 / 70);
        phase += 1;
    }
}
