/// API 接口集成测试
/// 测试 Tauri Commands 的功能

mod common;

use pastee_lib::persist::{Storage, ClipType};
use common::{create_test_dir, get_test_data_dir};
use std::sync::Mutex;

// 模拟 AppState（预留用于未来更复杂的测试）
#[allow(dead_code)]
struct TestAppState {
    storage: Mutex<Storage>,
}

#[allow(dead_code)]
impl TestAppState {
    fn new(storage: Storage) -> Self {
        Self {
            storage: Mutex::new(storage),
        }
    }
}

#[test]
fn test_get_recent_clips_command() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加测试数据
    storage.add_text("Test 1".to_string()).unwrap();
    storage.add_text("Test 2".to_string()).unwrap();
    storage.add_text("#FF0000".to_string()).unwrap();
    storage.add_text("#FF0055".to_string()).unwrap();
    
    // 测试获取最近记录
    let result = storage.get_recent(10, 0);
    assert!(result.is_ok(), "get_recent should succeed");
    
    let items = result.unwrap();
    assert_eq!(items.len(), 3, "Should have 3 items");
    
    // 验证顺序（最新的在前）
    assert_eq!(items[0].preview, "#FF0000");
    assert_eq!(items[1].preview, "Test 2");
    assert_eq!(items[2].preview, "Test 1");
}

#[test]
fn test_search_clips_command() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加测试数据
    storage.add_text("Rust is awesome".to_string()).unwrap();
    storage.add_text("Python is great".to_string()).unwrap();
    storage.add_text("Rust programming".to_string()).unwrap();
    
    // 搜索 "Rust"
    let result = storage.search("Rust");
    assert!(result.is_ok(), "search should succeed");
    
    let items = result.unwrap();
    assert_eq!(items.len(), 2, "Should find 2 items with 'Rust'");
}

#[test]
fn test_api_pagination() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加 20 条记录
    for i in 0..20 {
        storage.add_text(format!("Item {}", i)).unwrap();
    }
    
    // 测试分页
    let page1 = storage.get_recent(10, 0).unwrap();
    let page2 = storage.get_recent(10, 10).unwrap();
    
    assert_eq!(page1.len(), 10, "First page should have 10 items");
    assert_eq!(page2.len(), 10, "Second page should have 10 items");
    
    // 验证没有重复
    let ids1: Vec<i64> = page1.iter().map(|item| item.id).collect();
    let ids2: Vec<i64> = page2.iter().map(|item| item.id).collect();
    
    for id in &ids1 {
        assert!(!ids2.contains(id), "Pages should not have duplicate IDs");
    }
}

#[test]
fn test_api_error_handling() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let storage = Storage::new(&data_dir).unwrap();
    
    // 测试查询不存在的内容
    let result = storage.get_content(99999);
    assert!(result.is_err(), "Should return error for non-existent ID");
}

#[test]
fn test_concurrent_access() {
    use std::thread;
    use std::sync::Arc;
    
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let storage = Arc::new(Mutex::new(Storage::new(&data_dir).unwrap()));
    
    let mut handles = vec![];
    
    // 启动 5 个线程并发写入
    for i in 0..5 {
        let storage_clone = Arc::clone(&storage);
        let handle = thread::spawn(move || {
            let mut storage = storage_clone.lock().unwrap();
            for j in 0..10 {
                storage.add_text(format!("Thread {} - Item {}", i, j)).unwrap();
            }
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 验证所有记录都被写入
    let storage = storage.lock().unwrap();
    let all_items = storage.get_recent(100, 0).unwrap();
    assert_eq!(all_items.len(), 50, "Should have 50 items from 5 threads");
}

#[test]
fn test_api_content_types() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 测试所有内容类型
    storage.add_text("Plain text".to_string()).unwrap();
    storage.add_text("#FF0000".to_string()).unwrap();
    storage.add_html("Preview".to_string(), "<p>HTML</p>".to_string()).unwrap();
    
    let items = storage.get_recent(10, 0).unwrap();
    
    assert_eq!(items[0].content_type, ClipType::Html);
    assert_eq!(items[1].content_type, ClipType::Color);
    assert_eq!(items[2].content_type, ClipType::Text);
}
