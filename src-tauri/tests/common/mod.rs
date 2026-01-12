/// 测试公共模块
/// 提供测试辅助函数和工具

use std::path::PathBuf;
use tempfile::TempDir;

/// 创建临时测试目录
pub fn create_test_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

/// 获取测试数据目录路径
pub fn get_test_data_dir(temp_dir: &TempDir) -> PathBuf {
    temp_dir.path().to_path_buf()
}

/// 测试颜色值示例
#[allow(dead_code)]
pub fn test_color_samples() -> Vec<&'static str> {
    vec![
        "#F0F",
        "#FF00FF",
        "#FF00FF80",
        "rgb(255, 0, 255)",
        "rgba(255, 0, 255, 0.5)",
        "hsl(300, 100%, 50%)",
        "hsla(300, 100%, 50%, 0.5)",
    ]
}

/// 非颜色值示例
#[allow(dead_code)]
pub fn test_non_color_samples() -> Vec<&'static str> {
    vec![
        "Hello World",
        "#GG00FF",  // 非法字符
        "rgb(255, 0)",  // 参数不足
        "just text",
        "12345",
    ]
}

/// 清理测试数据
#[allow(dead_code)]
pub fn cleanup_test_db(path: PathBuf) {
    if path.exists() {
        let _ = std::fs::remove_file(path);
    }
}
