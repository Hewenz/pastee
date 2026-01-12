/// 中文搜索测试
/// 验证搜索功能对中文、日文、韩文等多字节字符的支持

mod common;

use pastee_lib::persist::Storage;
use common::{create_test_dir, get_test_data_dir};

#[test]
fn test_search_simple_chinese() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加中文内容
    storage.add_text("这是一个测试".to_string()).unwrap();
    storage.add_text("另一个中文内容".to_string()).unwrap();
    storage.add_text("English content".to_string()).unwrap();
    
    // 搜索中文
    let results = storage.search("测试").unwrap();
    assert_eq!(results.len(), 1, "应该找到1条包含'测试'的记录");
    assert!(results[0].preview.contains("测试"));
}

#[test]
fn test_search_chinese_phrase() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    storage.add_text("剪贴板管理工具".to_string()).unwrap();
    storage.add_text("文件管理器".to_string()).unwrap();
    storage.add_text("系统设置".to_string()).unwrap();
    
    // 搜索词组
    let results = storage.search("剪贴板").unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].preview.contains("剪贴板"));
    
    // 搜索部分匹配
    let results = storage.search("管理").unwrap();
    assert_eq!(results.len(), 2, "应该找到2条包含'管理'的记录");
}

#[test]
fn test_search_mixed_content() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    storage.add_text("Hello 你好 World 世界".to_string()).unwrap();
    storage.add_text("Rust 编程语言".to_string()).unwrap();
    storage.add_text("macOS 操作系统".to_string()).unwrap();
    
    // 搜索中文部分
    let results = storage.search("你好").unwrap();
    assert_eq!(results.len(), 1);
    
    // 搜索英文部分
    let results = storage.search("Rust").unwrap();
    assert_eq!(results.len(), 1);
    
    // 搜索中文
    let results = storage.search("编程").unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_japanese_korean() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 日文
    storage.add_text("こんにちは世界".to_string()).unwrap();
    // 韩文
    storage.add_text("안녕하세요 세계".to_string()).unwrap();
    // 中文
    storage.add_text("你好世界".to_string()).unwrap();
    
    // 搜索日文
    let results = storage.search("こんにちは").unwrap();
    assert_eq!(results.len(), 1);
    
    // 搜索韩文
    let results = storage.search("안녕").unwrap();
    assert_eq!(results.len(), 1);
    
    // 搜索中文
    let results = storage.search("你好").unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_chinese_punctuation() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    storage.add_text("你好，世界！".to_string()).unwrap();
    storage.add_text("测试：成功".to_string()).unwrap();
    storage.add_text("【重要】通知".to_string()).unwrap();
    
    // 搜索包含标点符号的中文
    let results = storage.search("你好").unwrap();
    assert_eq!(results.len(), 1);
    
    let results = storage.search("测试").unwrap();
    assert_eq!(results.len(), 1);
    
    let results = storage.search("重要").unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_chinese_numbers() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    storage.add_text("订单号：12345".to_string()).unwrap();
    storage.add_text("数量：100个".to_string()).unwrap();
    storage.add_text("价格：￥99.9".to_string()).unwrap();
    
    // 搜索中文
    let results = storage.search("订单").unwrap();
    assert_eq!(results.len(), 1);
    
    // 搜索数字
    let results = storage.search("12345").unwrap();
    assert_eq!(results.len(), 1);
    
    // 搜索符号
    let results = storage.search("￥").unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_long_chinese_text() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    let long_text = "这是一段很长的中文文本，用于测试搜索功能。\
                     它包含多个句子，每个句子都有不同的内容。\
                     我们希望能够准确地搜索到其中的关键词。\
                     比如：剪贴板、管理、工具、测试等等。";
    
    storage.add_text(long_text.to_string()).unwrap();
    storage.add_text("另一段文本".to_string()).unwrap();
    
    // 搜索长文本中的关键词
    let results = storage.search("剪贴板").unwrap();
    assert_eq!(results.len(), 1);
    
    let results = storage.search("关键词").unwrap();
    assert_eq!(results.len(), 1);
    
    let results = storage.search("测试").unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_chinese_html() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加包含中文的 HTML
    storage.add_html(
        "这是标题".to_string(),
        "<h1>这是标题</h1><p>这是内容</p>".to_string()
    ).unwrap();
    
    // 搜索中文内容
    let results = storage.search("标题").unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].preview.contains("标题"));
}

#[test]
fn test_search_empty_and_special() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    storage.add_text("正常文本".to_string()).unwrap();
    
    // 空搜索会匹配所有内容（LIKE '%%'）
    let results = storage.search("").unwrap();
    assert!(results.len() > 0, "空搜索应该返回所有内容");
    
    // 特殊字符搜索（已转义）
    storage.add_text("包含%符号".to_string()).unwrap();
    let results = storage.search("%").unwrap();
    assert_eq!(results.len(), 1, "应该能找到包含%的记录");
    
    storage.add_text("包含_下划线".to_string()).unwrap();
    let results = storage.search("_").unwrap();
    assert_eq!(results.len(), 1, "应该能找到包含_的记录");
}

#[test]
fn test_search_case_sensitivity() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    storage.add_text("Hello World".to_string()).unwrap();
    storage.add_text("HELLO WORLD".to_string()).unwrap();
    storage.add_text("hello world".to_string()).unwrap();
    
    // SQLite LIKE 默认不区分大小写（对于 ASCII）
    let results = storage.search("hello").unwrap();
    assert!(results.len() >= 1, "应该能找到包含 hello 的记录");
}
