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



// the Import block
mod file_manipulation;
mod theme_and_decoration;
use file_manipulation::*;
use theme_and_decoration::*;

use fltk::{prelude::*, *, enums::*};
use fltk_decl::{DeclarativeApp,Widget};
use std::env;

pub const OPERATING_SYSTEM: &str = env::consts::OS;
pub const APP_NAME: &str = "Wildfire Write";

fn main() {
    DeclarativeApp::new_json(1280, 720, "MyApp", "resources/gui/gui.json").run(|_win| 
        {
            //if let Some(mut choice) = app::widget_from_id::<menu::Choice>("choice")
            if let Some (mut wind) = app::widget_from_id::<window::DoubleWindow>("wind"){
            wind.set_color(Color::Black)
            }
        }).unwrap();
}
