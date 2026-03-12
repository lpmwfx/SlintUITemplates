/// Target platform for AppShell rendering.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Platform {
    #[default]
    Windows,
    Android,
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Windows => "windows",
            Platform::Android => "android",
        }
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, Platform::Android)
    }

    pub fn is_desktop(&self) -> bool {
        !self.is_mobile()
    }
}
