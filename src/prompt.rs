use std::path::Path;
use std::sync::LazyLock;

static USER_NAME: LazyLock<String> = LazyLock::new(whoami::username);
static DEVICE_NAME: LazyLock<String> = LazyLock::new(|| {
    let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string());
    hostname
        .strip_suffix(".local")
        .unwrap_or(&hostname)
        .to_string()
});

pub fn shell_prompt(curr_dir: &Path) -> String {
    let dir_name = curr_dir.file_name().and_then(|s| s.to_str()).unwrap_or("~");

    format!("{}@{} {dir_name} > ", *USER_NAME, *DEVICE_NAME)
}
