// Prefixes for info
// Set these to "" to disable
pub const OS: &str = "\x1b[1;34mOS:       \x1b[0m";
pub const UPTIME: &str = "\x1b[1;34mUptime:   \x1b[0m";
pub const KERNEL: &str = "\x1b[1;34mKernel:   \x1b[0m";
pub const SHELL: &str = "\x1b[1;34mShell:    \x1b[0m";
pub const DE: &str = "\x1b[1;34mDE:       \x1b[0m";
pub const PACKAGES: &str = "\x1b[1;34mPackages: \x1b[0m";
pub const CPU: &str = "\x1b[1;34mCPU:      \x1b[0m";
pub const MEM: &str = "\x1b[1;34mMemory:   \x1b[0m";

// Color toggles
pub const SHOW_REGULAR_COLORS: bool = true;
pub const SHOW_INTENSE_COLORS: bool = true;

// String used in the color display
pub const COLOR_STRING: &str = "\u{25cf} "; // ●
