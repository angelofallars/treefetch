pub mod codes {
    pub const _RED: &str = "\x1b[31m";
    pub const _GREEN: &str = "\x1b[32m";
    pub const _YELLOW: &str = "\x1b[33m";
    pub const _BLUE: &str = "\x1b[34m";
    pub const _CYAN: &str = "\x1b[36m";
    pub const _GRAY: &str = "\x1b[38;5;8m";
    pub const _MAGENTA: &str = "\x1b[35m";
    pub const _BOLD: &str = "\x1b[1m";
    pub const _RESET: &str = "\x1b[0m";
}

pub use self::codes::_RED as red;
pub use self::codes::_GREEN as green;
pub use self::codes::_YELLOW as yellow;
pub use self::codes::_BLUE as blue;
pub use self::codes::_CYAN as cyan;
pub use self::codes::_MAGENTA as magenta;
pub use self::codes::_GRAY as gray;
pub use self::codes::_BOLD as bold;
pub use self::codes::_RESET as reset;
