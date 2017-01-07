extern crate rand;
extern crate rustbox;
extern crate snake;
use std::default::Default;
use std::process::exit;
use rustbox::{Color, RustBox, Key};
use rand::Rng;
use snake::{direction, point, player};

fn show_intro(rb: &RustBox) {
    let ascii_art = vec![
    " ____              _        _",
    "/ ___| _ __   __ _| | _____| |",
    "\\___ \\| '_ \\ / _` | |/ / _ \\ |",
    " ___) | | | | (_| |   <  __/_|",
    "|____/|_| |_|\\__,_|_|\\_\\___(_)"];
    let art_width   = ascii_art[0].len() as i16;
    let art_height  = ascii_art.len() as i16;
    let term_width  = rb.width() as i16;
    let term_height = rb.height() as i16;
    let start_x = ((term_width - art_width) / 2) as i16;
    let start_y = ((term_height - art_height) / 2) as i16;
    for (i, y) in (start_y..(start_y + art_height)).enumerate() {
        rb.print(start_x as usize, y as usize, rustbox::RB_BOLD, Color::Green, Color::Black, ascii_art[i]);
    };
    let next_key_message = "Hit space to continue.";
    let quit_message = "Hit <esc> to exit.";
    rb.print(((term_width - next_key_message.len() as i16)/2) as usize, (term_height - 2) as usize,
             rustbox::RB_BOLD, Color::White, Color::Black, next_key_message);
    rb.print(((term_width - quit_message.len() as i16)/2) as usize, (term_height - 1) as usize,
             rustbox::RB_BOLD, Color::White, Color::Black, quit_message);
    loop {
        rb.present();
        match rb.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char(' ') => { break; },
                    Key::Esc => { panic!(); },
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}

fn draw_playfield(rb: &RustBox, start: point::Point, width: i16, height: i16) {
    // Draw the four corners
    rb.print(start.x as usize, start.y as usize, rustbox::RB_NORMAL,
             Color::White, Color::Black, "╔");
    rb.print((start.x + width) as usize, start.y as usize, rustbox::RB_NORMAL,
             Color::White, Color::Black, "╗");
    rb.print(start.x as usize, (start.y + height) as usize, rustbox::RB_NORMAL,
             Color::White, Color::Black, "╚");
    rb.print((start.x + width) as usize, (start.y + height) as usize, rustbox::RB_NORMAL,
             Color::White, Color::Black, "╝");
    for x in (start.x + 1)..width {
        rb.print(x as usize, start.y as usize, rustbox::RB_NORMAL,
                 Color::White, Color::Black, "═");
        rb.print(x as usize, (start.y + height) as usize, rustbox::RB_NORMAL,
                 Color::White, Color::Black, "═");
    }
    for y in (start.y + 1)..(height + 1) {
        rb.print(start.x as usize, y as usize, rustbox::RB_NORMAL,
                 Color::White, Color::Black, "║");
        rb.print((start.x + width) as usize, y as usize, rustbox::RB_NORMAL,
                 Color::White, Color::Black, "║");
    }
}
fn main() {
    let rb = match RustBox::init(Default::default()) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    show_intro(&rb);
    rb.clear();
    loop {
        draw_playfield(&rb, point::Point::new(0, 1), (rb.width() - 1) as i16, (rb.height() - 2) as i16);
        rb.present();
        match rb.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; },
                    Key::Esc => { break; },
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}
