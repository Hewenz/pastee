/// IPC 接口集成测试
/// 测试所有 Tauri 命令的功能

mod common;

use pastee_lib::persist::Storage;
use common::{create_test_dir, get_test_data_dir};
use std::sync::Mutex;

// 模拟 AppState
struct TestAppState {
    storage: Mutex<Storage>,
}

impl TestAppState {
    fn new(storage: Storage) -> Self {
        Self {
            storage: Mutex::new(storage),
        }
    }
}

#[test]
fn test_ipc_get_recent_clips() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加测试数据
    storage.add_text("Clip 1".to_string()).unwrap();
    storage.add_text("Clip 2".to_string()).unwrap();
    storage.add_text("Clip 3".to_string()).unwrap();
    
    // 模拟 IPC 命令：get_recent_clips(limit=2, offset=0)
    let result = storage.get_recent(2, 0);
    assert!(result.is_ok(), "get_recent_clips should succeed");
    
    let items = result.unwrap();
    assert_eq!(items.len(), 2, "Should return 2 items");
    assert_eq!(items[0].preview, "Clip 3", "Most recent should be first");
}

#[test]
fn test_ipc_search_clips() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加测试数据
    storage.add_text("Hello World".to_string()).unwrap();
    storage.add_text("Rust Programming".to_string()).unwrap();
    storage.add_text("Hello Rust".to_string()).unwrap();
    
    // 模拟 IPC 命令：search_clips(query="Rust")
    let result = storage.search("Rust");
    assert!(result.is_ok(), "search_clips should succeed");
    
    let items = result.unwrap();
    assert_eq!(items.len(), 2, "Should find 2 items with 'Rust'");
}

#[test]
fn test_ipc_get_clip_content() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加不同类型的内容
    storage.add_text("Simple text".to_string()).unwrap();
    storage.add_html("HTML Preview".to_string(), "<p>HTML Content</p>".to_string()).unwrap();
    
    let items = storage.get_recent(2, 0).unwrap();
    
    // 获取 HTML 内容
    let html_id = items.iter().find(|i| i.content_type == pastee_lib::persist::ClipType::Html)
        .map(|i| i.id)
        .expect("Should have HTML item");
    
    let content = storage.get_content(html_id);
    assert!(content.is_ok(), "get_clip_content should succeed");
    
    match content.unwrap() {
        pastee_lib::persist::ClipData::Html { text, html } => {
            assert_eq!(text, "HTML Preview");
            assert_eq!(html, "<p>HTML Content</p>");
        },
        _ => panic!("Should return HTML ClipData"),
    }
}

#[test]
fn test_ipc_toggle_pin() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let storage = Storage::new(&data_dir).unwrap();
    let mut storage = storage;
    
    // 添加测试项
    storage.add_text("Item to pin".to_string()).unwrap();
    
    let items = storage.get_recent(1, 0).unwrap();
    let item_id = items[0].id;
    
    // 初始状态应该是未置顶
    assert!(!items[0].is_pinned, "Item should not be pinned initially");
    
    // 模拟 IPC 命令：toggle_pin(id)
    let new_state = storage.toggle_pin(item_id).unwrap();
    assert!(new_state, "Should return true after pinning");
    
    // 验证状态改变
    let items = storage.get_recent(1, 0).unwrap();
    assert!(items[0].is_pinned, "Item should be pinned");
    
    // 再次切换
    let new_state = storage.toggle_pin(item_id).unwrap();
    assert!(!new_state, "Should return false after unpinning");
}

#[test]
fn test_ipc_delete_clip() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加测试项
    storage.add_text("Item to delete".to_string()).unwrap();
    storage.add_text("Item to keep".to_string()).unwrap();
    
    let items = storage.get_recent(2, 0).unwrap();
    assert_eq!(items.len(), 2, "Should have 2 items");
    
    let item_to_delete_id = items.iter()
        .find(|i| i.preview == "Item to delete")
        .map(|i| i.id)
        .expect("Should find item");
    
    // 模拟 IPC 命令：delete_clip(id)
    let result = storage.delete_record(item_to_delete_id);
    assert!(result.is_ok(), "delete_clip should succeed");
    
    // 验证项被删除
    let items = storage.get_recent(10, 0).unwrap();
    assert_eq!(items.len(), 1, "Should have 1 item after deletion");
    assert_eq!(items[0].preview, "Item to keep", "Correct item should remain");
}

#[test]
fn test_ipc_pinned_order() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加多个项
    storage.add_text("First".to_string()).unwrap();
    storage.add_text("Second".to_string()).unwrap();
    storage.add_text("Third".to_string()).unwrap();
    
    // 置顶第一个
    let items = storage.get_recent(10, 0).unwrap();
    let first_id = items.iter().find(|i| i.preview == "First").map(|i| i.id).unwrap();
    storage.toggle_pin(first_id).unwrap();
    
    // 验证置顶的项排在前面
    let items = storage.get_recent(10, 0).unwrap();
    assert!(items[0].is_pinned, "First item in list should be pinned");
    assert_eq!(items[0].preview, "First", "Pinned item should appear first");
}

#[test]
fn test_ipc_pagination() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加 15 个项
    for i in 0..15 {
        storage.add_text(format!("Item {}", i)).unwrap();
    }
    
    // 第一页
    let page1 = storage.get_recent(5, 0).unwrap();
    assert_eq!(page1.len(), 5, "First page should have 5 items");
    
    // 第二页
    let page2 = storage.get_recent(5, 5).unwrap();
    assert_eq!(page2.len(), 5, "Second page should have 5 items");
    
    // 第三页
    let page3 = storage.get_recent(5, 10).unwrap();
    assert_eq!(page3.len(), 5, "Third page should have 5 items");
    
    // 检查没有重复
    let all_ids: Vec<i64> = page1.iter().map(|i| i.id).collect();
    for item in &page2 {
        assert!(!all_ids.contains(&item.id), "No duplicate IDs");
    }
}

#[test]
fn test_ipc_error_handling() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let storage = Storage::new(&data_dir).unwrap();
    
    // 尝试获取不存在的内容
    let result = storage.get_content(99999);
    assert!(result.is_err(), "Should return error for non-existent ID");
}

#[test]
fn test_ipc_color_with_search() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加混合类型
    storage.add_text("Text content".to_string()).unwrap();
    storage.add_text("#FF0000".to_string()).unwrap();
    storage.add_text("Another text".to_string()).unwrap();
    
    // 搜索颜色 - 使用完整的颜色值
    let results = storage.search("FF0000").unwrap();
    assert_eq!(results.len(), 1, "Should find color by hex");
    
    // 验证是色彩类型
    let items = storage.get_recent(10, 0).unwrap();
    let color_item = items.iter().find(|i| i.preview == "#FF0000").unwrap();
    assert_eq!(color_item.content_type, pastee_lib::persist::ClipType::Color);
}
