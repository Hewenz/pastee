/// 数据库集成测试
/// 测试 Storage 层的所有功能

mod common;

use pastee_lib::persist::{Storage, ClipType, ClipData};
use common::{create_test_dir, get_test_data_dir, test_color_samples, test_non_color_samples};

#[test]
fn test_storage_initialization() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    
    let storage = Storage::new(&data_dir);
    assert!(storage.is_ok(), "Storage should initialize successfully");
    
    // 验证数据库文件存在
    assert!(data_dir.join("clippy.db").exists(), "Database file should exist");
    
    // 验证图片目录存在
    assert!(data_dir.join("images").exists(), "Images directory should exist");
}

#[test]
fn test_add_text() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let text = "Hello, World!".to_string();
    let result = storage.add_text(text.clone());
    
    assert!(result.is_ok(), "add_text should succeed");
    let id = result.unwrap();
    assert!(id > 0, "ID should be positive");
    
    // 验证可以检索
    let recent = storage.get_recent(10, 0).unwrap();
    assert_eq!(recent.len(), 1, "Should have one record");
    assert_eq!(recent[0].preview, text, "Preview should match input");
    assert_eq!(recent[0].content_type, ClipType::Text, "Type should be Text");
    assert!(recent[0].tags.contains(&"text".to_string()), "Should have 'text' tag");
}

#[test]
fn test_add_color() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let mut added_count = 0;
    for color in test_color_samples() {
        let result = storage.add_text(color.to_string());
        assert!(result.is_ok(), "add_text should succeed for color: {}", color);
        added_count += 1;
    }
    
    // 获取所有添加的颜色
    let all_items = storage.get_recent(added_count, 0).unwrap();
    
    // 所有项目都应该是 Color 类型
    for item in &all_items {
        assert_eq!(item.content_type, ClipType::Color, "Type should be Color for: {}", item.preview);
        assert!(item.tags.contains(&"color".to_string()), "Should have 'color' tag for: {}", item.preview);
    }
}

#[test]
fn test_color_detection() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 测试非颜色值应该被识别为 Text
    for non_color in test_non_color_samples() {
        let result = storage.add_text(non_color.to_string());
        assert!(result.is_ok(), "add_text should succeed for: {}", non_color);
        
        let recent = storage.get_recent(1, 0).unwrap();
        assert_eq!(recent[0].content_type, ClipType::Text, "Should be Text for: {}", non_color);
        assert!(recent[0].tags.contains(&"text".to_string()), "Should have 'text' tag for: {}", non_color);
    }
}

#[test]
fn test_deduplication() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let text = "Duplicate Test".to_string();
    
    // 添加第一次
    let id1 = storage.add_text(text.clone()).unwrap();
    
    // 添加第二次（相同内容）
    let id2 = storage.add_text(text.clone()).unwrap();
    
    // 应该返回相同的 ID（去重）
    assert_eq!(id1, id2, "Duplicate content should return same ID");
    
    // 列表中应该只有一条记录
    let recent = storage.get_recent(10, 0).unwrap();
    assert_eq!(recent.len(), 1, "Should only have one record after deduplication");
}

#[test]
fn test_add_html() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let text_preview = "Hello World".to_string();
    let html_content = "<p>Hello <strong>World</strong></p>".to_string();
    
    let result = storage.add_html(text_preview.clone(), html_content.clone());
    assert!(result.is_ok(), "add_html should succeed");
    
    let id = result.unwrap();
    let recent = storage.get_recent(1, 0).unwrap();
    
    assert_eq!(recent[0].content_type, ClipType::Html, "Type should be Html");
    assert!(recent[0].tags.contains(&"html".to_string()), "Should have 'html' tag");
    
    // 验证可以获取完整内容
    let content = storage.get_content(id).unwrap();
    match content {
        ClipData::Html { text, html } => {
            assert_eq!(text, text_preview, "Text preview should match");
            assert_eq!(html, html_content, "HTML content should match");
        },
        _ => panic!("Should return Html ClipData"),
    }
}

#[test]
fn test_search() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加多条记录
    storage.add_text("Rust programming".to_string()).unwrap();
    storage.add_text("Python programming".to_string()).unwrap();
    storage.add_text("JavaScript development".to_string()).unwrap();
    
    // 搜索 "programming"
    let results = storage.search("programming").unwrap();
    assert_eq!(results.len(), 2, "Should find 2 records with 'programming'");
    
    // 搜索 "Rust"
    let results = storage.search("Rust").unwrap();
    assert_eq!(results.len(), 1, "Should find 1 record with 'Rust'");
}

#[test]
fn test_pagination() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加 10 条记录
    for i in 0..10 {
        storage.add_text(format!("Record {}", i)).unwrap();
    }
    
    // 第一页（5条）
    let page1 = storage.get_recent(5, 0).unwrap();
    assert_eq!(page1.len(), 5, "First page should have 5 records");
    
    // 第二页（5条）
    let page2 = storage.get_recent(5, 5).unwrap();
    assert_eq!(page2.len(), 5, "Second page should have 5 records");
    
    // 确保不重复
    assert_ne!(page1[0].id, page2[0].id, "Pages should have different records");
}

#[test]
fn test_tag_array() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加颜色
    storage.add_text("#FF0000".to_string()).unwrap();
    
    let recent = storage.get_recent(1, 0).unwrap();
    
    // 验证 tags 是数组
    assert!(recent[0].tags.len() > 0, "Should have at least one tag");
    assert!(recent[0].tags.contains(&"color".to_string()), "Should contain 'color' tag");
    
    // 添加文本
    storage.add_text("Hello World".to_string()).unwrap();
    
    let all_items = storage.get_recent(10, 0).unwrap();
    
    assert_eq!(all_items.len(), 2, "Should have 2 items");
    
    // 检查两个项目的类型和标签（不依赖于顺序）
    let color_item = all_items.iter().find(|item| item.preview == "#FF0000").expect("Should have color item");
    let text_item = all_items.iter().find(|item| item.preview == "Hello World").expect("Should have text item");
    
    assert_eq!(color_item.content_type, ClipType::Color, "Color item should be ClipType::Color");
    assert!(color_item.tags.contains(&"color".to_string()), "Color item should have 'color' tag");
    
    assert_eq!(text_item.content_type, ClipType::Text, "Text item should be ClipType::Text");
    assert!(text_item.tags.contains(&"text".to_string()), "Text item should have 'text' tag");
}

#[test]
fn test_empty_text() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 空文本应该被忽略
    let result = storage.add_text("".to_string()).unwrap();
    assert_eq!(result, 0, "Empty text should return 0");
    
    // 只有空白的文本也应该被忽略
    let result = storage.add_text("   ".to_string()).unwrap();
    assert_eq!(result, 0, "Whitespace-only text should return 0");
}

#[test]
fn test_database_migration() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    
    // 第一次初始化
    {
        let _storage = Storage::new(&data_dir).unwrap();
    }
    
    // 第二次初始化（应该使用已有数据库）
    let storage2 = Storage::new(&data_dir);
    assert!(storage2.is_ok(), "Should reopen existing database successfully");
}
