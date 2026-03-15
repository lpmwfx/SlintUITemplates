/// Target platform for AppShell rendering and token routing.
#[derive(Debug, Clone, PartialEq, Default)]
/// P la tf or m enum.
pub enum Platform {
    #[default]
    Windows,
    Android,
    /// Steam Deck — 1280×800 touchscreen + gamepad primary, small tier.
    SteamDeck,
    /// Steam Linux — GamepadUI fullscreen, desktop-class hardware, small tier.
    SteamLinux,
}

impl Platform {
    /// Returns the platform identifier string that matches `Variants.*` in Slint.
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Windows    => "windows",
            Platform::Android    => "android",
            Platform::SteamDeck  => "steam-deck",
            Platform::SteamLinux => "steam-linux",
        }
    }

    /// Returns `true` if the platform is a mobile target (portrait touch-first).
    pub fn is_mobile(&self) -> bool {
        matches!(self, Platform::Android)
    }

    /// Returns `true` if the platform is the small tier (landscape touch + gamepad).
    pub fn is_small(&self) -> bool {
        matches!(self, Platform::SteamDeck | Platform::SteamLinux)
    }

    /// Returns `true` if the platform is a conventional desktop target.
    pub fn is_desktop(&self) -> bool {
        !self.is_mobile() && !self.is_small()
    }
}
