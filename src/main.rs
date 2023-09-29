/*
Wildfire Write, a free and open source writing software to keep your worldbuilding organized, inspired by campfire write and world anvil.
(C) 2023
Nadichamp
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

mod file_manipulation;
use file_manipulation::*;

mod menu_stuff;
use menu_stuff::*;


use fltk::{prelude::*, *, enums::*, text::TextEditor, utils::oncelock::Lazy, menu::SysMenuBar};
use std::{env, path::*};

const HEIGHT: i32 = 900;
const WIDTH: i32 = 1600;
pub const OPERATING_SYSTEM: &str = env::consts::OS;
pub const APP_NAME: &str = "Wildfire Write";

static STATE: Lazy<app::GlobalState<FileState>> = Lazy::new(app::GlobalState::<FileState>::get);
pub struct FileState {
    pub saved: bool,
    pub buf: text::TextBuffer,
    pub current_file: PathBuf,
    pub name: Option<String>
}

impl FileState {
    fn new(buf: text::TextBuffer) -> Self {
        FileState {
            saved: true,
            buf,
            current_file: PathBuf::new(),
            name: dialog::input(800, 450, "Name the file", "")
        }
    }
}


fn main() {
    println!("Wildfire Write (C) 2023 nadichamp");
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let mut wind = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Wildfire Write");

    let mut bar: SysMenuBar = SysMenuBar::new(0,0, WIDTH, 25, "Wildfire Write");

    init_menu(&mut bar);

    let mut side_bar_wind = window::Window::new(0,25,250,HEIGHT, "");
        &side_bar_wind.set_color(Color::Black);


    side_bar_wind.end();

    wind.end();
    wind.show();

    let buf = text::TextBuffer{};
    let file = FileState::new(buf);
    wind.set_callback(wind_cb);
    app.run().unwrap();
}

fn wind_cb(_w: &mut window::Window) {
    if app::event() == Event::Close {
        quit_cb();
    }
}

fn quit_cb () {
    app::quit();
}

