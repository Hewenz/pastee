/// 颜色检测算法测试
/// 详细测试各种颜色格式的识别

mod common;

use pastee_lib::persist::Storage;
use common::{create_test_dir, get_test_data_dir};

#[test]
fn test_hex_3_digit() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let hex_colors = vec!["#F0F", "#ABC", "#123", "#000", "#FFF"];
    
    for color in hex_colors {
        let _result = storage.add_text(color.to_string()).unwrap();
        let items = storage.get_recent(1, 0).unwrap();
        
        assert!(
            items[0].tags.contains(&"color".to_string()),
            "HEX-3 '{}' should be detected as color", 
            color
        );
    }
}

#[test]
fn test_hex_6_digit() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let hex_colors = vec!["#FF00FF", "#00FF00", "#0000FF", "#ABCDEF", "#123456"];
    
    for color in hex_colors {
        storage.add_text(color.to_string()).unwrap();
        let items = storage.get_recent(1, 0).unwrap();
        
        assert!(
            items[0].tags.contains(&"color".to_string()),
            "HEX-6 '{}' should be detected as color", 
            color
        );
    }
}

#[test]
fn test_hex_8_digit() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let hex_colors = vec!["#FF00FF80", "#00FF00FF", "#0000FF00", "#ABCDEF12"];
    
    for color in hex_colors {
        storage.add_text(color.to_string()).unwrap();
        let items = storage.get_recent(1, 0).unwrap();
        
        assert!(
            items[0].tags.contains(&"color".to_string()),
            "HEX-8 '{}' should be detected as color", 
            color
        );
    }
}

#[test]
fn test_rgb_format() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let rgb_colors = vec![
        "rgb(255, 0, 255)",
        "rgb(0, 255, 0)",
        "rgb(128, 128, 128)",
        "RGB(255, 0, 0)",  // 大写
        "rgb(0,0,0)",      // 无空格
    ];
    
    for color in rgb_colors {
        storage.add_text(color.to_string()).unwrap();
        let items = storage.get_recent(1, 0).unwrap();
        
        assert!(
            items[0].tags.contains(&"color".to_string()),
            "RGB '{}' should be detected as color", 
            color
        );
    }
}

#[test]
fn test_rgba_format() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let rgba_colors = vec![
        "rgba(255, 0, 255, 0.5)",
        "rgba(0, 255, 0, 1)",
        "RGBA(128, 128, 128, 0.7)",
    ];
    
    for color in rgba_colors {
        storage.add_text(color.to_string()).unwrap();
        let items = storage.get_recent(1, 0).unwrap();
        
        assert!(
            items[0].tags.contains(&"color".to_string()),
            "RGBA '{}' should be detected as color", 
            color
        );
    }
}

#[test]
fn test_hsl_format() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let hsl_colors = vec![
        "hsl(300, 100%, 50%)",
        "hsl(0, 0%, 50%)",
        "HSL(180, 50%, 75%)",
    ];
    
    for color in hsl_colors {
        storage.add_text(color.to_string()).unwrap();
        let items = storage.get_recent(1, 0).unwrap();
        
        assert!(
            items[0].tags.contains(&"color".to_string()),
            "HSL '{}' should be detected as color", 
            color
        );
    }
}

#[test]
fn test_hsla_format() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let hsla_colors = vec![
        "hsla(300, 100%, 50%, 0.5)",
        "hsla(0, 0%, 50%, 1)",
        "HSLA(180, 50%, 75%, 0.8)",
    ];
    
    for color in hsla_colors {
        storage.add_text(color.to_string()).unwrap();
        let items = storage.get_recent(1, 0).unwrap();
        
        assert!(
            items[0].tags.contains(&"color".to_string()),
            "HSLA '{}' should be detected as color", 
            color
        );
    }
}

#[test]
fn test_invalid_hex() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let invalid_hex = vec![
        "#GG00FF",     // 非法字符
        "#F",          // 长度错误
        "#FF",         // 长度错误
        "#FFFF",       // 长度错误
        "#FFFFF",      // 长度错误
        "#FFFFFFF",    // 长度错误
        "FF00FF",      // 缺少 #
    ];
    
    for invalid in invalid_hex {
        storage.add_text(invalid.to_string()).unwrap();
        let items = storage.get_recent(1, 0).unwrap();
        
        assert!(
            items[0].tags.contains(&"text".to_string()),
            "Invalid HEX '{}' should NOT be detected as color", 
            invalid
        );
    }
}

#[test]
fn test_invalid_rgb() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let invalid_rgb = vec![
        "rgb(255, 0)",         // 参数不足
        "rgb(255, 0, 255, 0)", // 参数过多
        "rgb(255 0 255)",      // 缺少逗号
        "rgb 255, 0, 255",     // 缺少括号
    ];
    
    for invalid in invalid_rgb {
        storage.add_text(invalid.to_string()).unwrap();
        let items = storage.get_recent(1, 0).unwrap();
        
        assert!(
            items[0].tags.contains(&"text".to_string()),
            "Invalid RGB '{}' should NOT be detected as color", 
            invalid
        );
    }
}

#[test]
fn test_whitespace_handling() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let colors_with_whitespace = vec![
        "  #FF00FF  ",
        "\t#FF00FF\t",
        " rgb(255, 0, 255) ",
    ];
    
    for color in colors_with_whitespace {
        storage.add_text(color.to_string()).unwrap();
        let items = storage.get_recent(1, 0).unwrap();
        
        assert!(
            items[0].tags.contains(&"color".to_string()),
            "Color with whitespace '{}' should be detected", 
            color
        );
    }
}

#[test]
fn test_case_insensitive() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let case_variants = vec![
        "rgb(255, 0, 255)",
        "RGB(255, 0, 255)",
        "Rgb(255, 0, 255)",
        "hsl(300, 100%, 50%)",
        "HSL(300, 100%, 50%)",
    ];
    
    for color in case_variants {
        storage.add_text(color.to_string()).unwrap();
        let items = storage.get_recent(1, 0).unwrap();
        
        assert!(
            items[0].tags.contains(&"color".to_string()),
            "Case variant '{}' should be detected as color", 
            color
        );
    }
}
