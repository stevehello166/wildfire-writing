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

use fltk::{prelude::*, *, enums::*};

pub fn init_menu(m: &mut menu::SysMenuBar) {
    m.add(
        "&File/New\t",
        Shortcut::Ctrl | 'n',
        menu::MenuFlag::Normal,
        menu_cb,
    );
    m.add(
        "&File/Open\t",
        Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        menu_cb,
    );
    m.add(
        "&File/Save\t",
        Shortcut::Ctrl | 's',
        menu::MenuFlag::Normal,
        menu_cb,
    );
    m.add(
        "&File/Save as\t",
        Shortcut::Ctrl | 'w',
        menu::MenuFlag::MenuDivider,
        menu_cb,
    );
    let idx = m.add(
        "&File/Quit\t",
        Shortcut::Ctrl | 'q',
        menu::MenuFlag::Normal,
        menu_cb,
    );
    m.at(idx).unwrap().set_label_color(Color::Red);
    m.add(
        "&Edit/Cut\t",
        Shortcut::Ctrl | 'x',
        menu::MenuFlag::Normal,
        menu_cb,
    );
    m.add(
        "&Edit/Copy\t",
        Shortcut::Ctrl | 'c',
        menu::MenuFlag::Normal,
        menu_cb,
    );
    m.add(
        "&Edit/Paste\t",
        Shortcut::Ctrl | 'v',
        menu::MenuFlag::Normal,
        menu_cb,
    );
    m.add(
        "&Help/About\t",
        Shortcut::None,
        menu::MenuFlag::Normal,
        menu_cb,
    );
    m.add(
        "&Help/License\t",
        Shortcut::Ctrl | 'l',
        menu::MenuFlag::Normal,
        menu_cb,
    );
}

pub fn menu_cb(m: &mut impl MenuExt) {
    if let Some(choice) = m.choice() {
        match choice.as_str() {
            "New\t" => println!("da"),
            "Insert" => println!("Insert"), // This will eventually be able to insert images and other things into documents
            "Quit\t" => {
                println!("Quitting");
                app::quit();
            },
            //displays about information
            "About\t" => {
                let mut help = dialog::HelpDialog::new(100, 100, 600, 300);
                //help.set_value(); // this takes html
                dialog::HelpDialog::load(&mut help, "resources/help/about.html");
                help.show();
                while help.shown() {
                    app::wait();
                }
            },
            //displays License
            "License\t" => {
                let mut license = dialog::HelpDialog::new(100, 100, 1600, 900);
                dialog::HelpDialog::load(&mut license, "resources/help/license.html");
                license.show();
                while license.shown() {
                    app::wait();
                }
            }
            _ => println!("{}", choice),
        }
    }
}
