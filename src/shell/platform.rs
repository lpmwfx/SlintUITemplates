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
}
