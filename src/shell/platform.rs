/// Target platform for AppShell rendering.
#[derive(Debug, Clone, PartialEq, Default)]
/// P la tf or m enum.
pub enum Platform {
    #[default]
    Windows,
    Android,
}

impl Platform {
    /// Returns the platform name as a lowercase string slice.
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Windows => "windows",
            Platform::Android => "android",
        }
    }

    /// Returns `true` if the platform is a mobile target (e.g. Android).
    pub fn is_mobile(&self) -> bool {
        matches!(self, Platform::Android)
    }

    /// Returns `true` if the platform is a desktop target (e.g. Windows).
    pub fn is_desktop(&self) -> bool {
        !self.is_mobile()
    }
}
