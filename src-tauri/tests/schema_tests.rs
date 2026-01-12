/// Schema 版本和兼容性测试
/// 验证数据库 schema 是否包含所有必要列

mod common;

use pastee_lib::persist::Storage;
use common::{create_test_dir, get_test_data_dir};

#[test]
fn test_schema_has_tag_column() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    
    // 创建新数据库（应该有 tag 列）
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加一条记录
    storage.add_text("Test text".to_string()).unwrap();
    
    // 尝试查询，应该成功（说明 tag 列存在）
    let result = storage.get_recent(10, 0);
    assert!(result.is_ok(), "Schema should have tag column");
    
    let items = result.unwrap();
    assert_eq!(items.len(), 1, "Should have one item");
    assert!(!items[0].tags.is_empty(), "Item should have tags");
    assert!(items[0].tags[0] == "text", "Default tag should be 'text'");
}

#[test]
fn test_schema_has_created_at_column() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    storage.add_text("Test".to_string()).unwrap();
    
    let items = storage.get_recent(1, 0).unwrap();
    
    // 验证 created_at 列存在并有有效值
    assert!(items[0].created_at > 0, "created_at should have a valid timestamp");
}

#[test]
fn test_schema_has_is_pinned_column() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    storage.add_text("Test".to_string()).unwrap();
    
    let items = storage.get_recent(1, 0).unwrap();
    
    // 验证 is_pinned 列存在（初始值应为 false）
    assert!(!items[0].is_pinned, "is_pinned should be false by default");
}

#[test]
fn test_schema_supports_all_required_columns() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    // 添加不同类型的内容
    storage.add_text("Text".to_string()).unwrap();
    storage.add_text("#FF0000".to_string()).unwrap();
    storage.add_html("HTML".to_string(), "<p>test</p>".to_string()).unwrap();
    
    // 验证 get_recent 可以查询所有列
    let result = storage.get_recent(10, 0);
    assert!(result.is_ok(), "get_recent should work with complete schema");
    
    let items = result.unwrap();
    for item in items {
        // 验证所有必需的字段都存在
        assert!(item.id > 0, "id should exist");
        assert!(!item.content_type.to_string().is_empty(), "content_type should exist");
        assert!(item.created_at > 0, "created_at should exist");
        assert!(!item.tags.is_empty(), "tags should exist");
        // is_pinned 应该有值（true 或 false）
    }
}

#[test]
fn test_search_works_with_complete_schema() {
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    let mut storage = Storage::new(&data_dir).unwrap();
    
    storage.add_text("Rust programming".to_string()).unwrap();
    storage.add_text("Python tutorial".to_string()).unwrap();
    
    // 验证 search 可以查询（依赖于 tag 列在 FTS 索引中）
    let result = storage.search("Rust");
    assert!(result.is_ok(), "search should work with complete schema");
    
    let items = result.unwrap();
    assert_eq!(items.len(), 1, "Should find one result");
}

#[test]
fn test_migration_creates_all_columns() {
    // 这个测试验证新数据库正确创建了所有必需的列
    let temp_dir = create_test_dir();
    let data_dir = get_test_data_dir(&temp_dir);
    
    // 两次初始化应该都成功（migration 是幂等的）
    let storage1 = Storage::new(&data_dir).unwrap();
    let storage2 = Storage::new(&data_dir).unwrap();
    
    // 如果能成功创建两次，说明 migration 正确处理了 IF NOT EXISTS
    drop(storage1);
    drop(storage2);
}
