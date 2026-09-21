#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Midnight,
    Dracula,
    TokyoNight,
    ClaudeCode,
    Gpt,
    Solarized,
    Adwaita,
}

impl Theme {
    pub fn name(self) -> &'static str {
        match self {
            Theme::Midnight => "Midnight",
            Theme::Dracula => "Dracula",
            Theme::TokyoNight => "Tokyo Night",
            Theme::ClaudeCode => "Claude Code",
            Theme::Gpt => "ChatGPT",
            Theme::Solarized => "Solarized",
            Theme::Adwaita => "Adwaita",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Theme::Midnight => "theme theme-midnight",
            Theme::Dracula => "theme theme-dracula",
            Theme::TokyoNight => "theme theme-tokyo",
            Theme::ClaudeCode => "theme theme-claude",
            Theme::Gpt => "theme theme-light theme-gpt",
            Theme::Solarized => "theme theme-light theme-solarized",
            Theme::Adwaita => "theme theme-light theme-adwaita",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Theme::Midnight => Theme::Dracula,
            Theme::Dracula => Theme::TokyoNight,
            Theme::TokyoNight => Theme::ClaudeCode,
            Theme::ClaudeCode => Theme::Gpt,
            Theme::Gpt => Theme::Solarized,
            Theme::Solarized => Theme::Adwaita,
            Theme::Adwaita => Theme::Midnight,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cycle_names() -> Vec<&'static str> {
        let mut theme = Theme::Midnight;
        let mut names = vec![theme.name()];
        theme = theme.next();
        while theme != Theme::Midnight {
            names.push(theme.name());
            theme = theme.next();
            assert!(names.len() < 20, "theme cycle did not return to Midnight");
        }
        names
    }

    #[test]
    fn cycle_includes_claude_gpt_and_linux_lights() {
        assert_eq!(
            cycle_names(),
            [
                "Midnight",
                "Dracula",
                "Tokyo Night",
                "Claude Code",
                "ChatGPT",
                "Solarized",
                "Adwaita",
            ]
        );
    }

    #[test]
    fn light_themes_use_theme_light_class() {
        let mut theme = Theme::Midnight;
        let mut lights = Vec::new();
        loop {
            if theme.class().contains("theme-light") {
                lights.push(theme.name());
            }
            theme = theme.next();
            if theme == Theme::Midnight {
                break;
            }
        }
        assert_eq!(lights, ["ChatGPT", "Solarized", "Adwaita"]);
    }

    #[test]
    fn claude_code_is_dark_with_claude_class() {
        let mut theme = Theme::Midnight;
        loop {
            if theme.name() == "Claude Code" {
                assert!(!theme.class().contains("theme-light"));
                assert!(theme.class().contains("theme-claude"));
                return;
            }
            theme = theme.next();
            if theme == Theme::Midnight {
                panic!("cycle missing Claude Code");
            }
        }
    }
}
