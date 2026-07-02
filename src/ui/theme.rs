use ratatui::style::Color;

pub struct Theme {
    pub light_square: Color,
    pub dark_square: Color,
    pub selection: Color,
    pub move_hint: Color,
    pub capture_hint: Color,
    pub last_move: Color,
    pub check: Color,
    pub text: Color,
    pub background: Color,
}

pub fn tokio_night_theme() -> Theme {
    Theme {
        light_square: Color::Rgb(46, 52, 64),   // #2e3440
        dark_square: Color::Rgb(59, 66, 82),    // #3b4252
        selection: Color::Rgb(94, 129, 172),    // #5e81ac
        move_hint: Color::Rgb(163, 190, 140),   // #a3be8c
        capture_hint: Color::Rgb(191, 97, 106), // #bf616a
        last_move: Color::Rgb(235, 203, 139),   // #ebcb8b
        check: Color::Rgb(208, 135, 112),       // #d08770
        text: Color::Rgb(216, 222, 233),        // #d8dee9
        background: Color::Rgb(30, 31, 41),     // overall bg
    }
}
