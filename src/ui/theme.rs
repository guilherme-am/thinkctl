#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Midnight,
    Dracula,
    TokyoNight,
}

impl Theme {
    pub fn name(self) -> &'static str {
        match self {
            Theme::Midnight => "Midnight",
            Theme::Dracula => "Dracula",
            Theme::TokyoNight => "Tokyo Night",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Theme::Midnight => "theme theme-midnight",
            Theme::Dracula => "theme theme-dracula",
            Theme::TokyoNight => "theme theme-tokyo",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Theme::Midnight => Theme::Dracula,
            Theme::Dracula => Theme::TokyoNight,
            Theme::TokyoNight => Theme::Midnight,
        }
    }
}
