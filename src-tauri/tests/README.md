# 测试框架说明

## 测试结构

已为 Pastee 项目建立完整的测试框架，包括：

### 1. 测试目录结构
```
src-tauri/tests/
├── common/
│   └── mod.rs          # 公共测试辅助函数
├── db_tests.rs         # 数据库功能测试
├── api_tests.rs        # API接口测试
└── color_detection_tests.rs  # 颜色检测测试
```

### 2. 依赖项
已在 `Cargo.toml` 添加测试依赖：
```toml
[dev-dependencies]
tempfile = "3.8.1"
```

## 测试覆盖

### 数据库测试 (db_tests.rs)
- ✅ **test_storage_initialization** - 存储初始化
- ✅ **test_add_text** - 添加文本
- ✅ **test_add_color** - 添加颜色
- ✅ **test_color_detection** - 颜色检测
- ✅ **test_deduplication** - 去重功能
- ✅ **test_add_html** - 添加HTML
- ✅ **test_search** - 搜索功能
- ✅ **test_pagination** - 分页功能
- ✅ **test_tag_array** - 标签数组
- ✅ **test_empty_text** - 空文本处理
- ✅ **test_database_migration** - 数据库迁移

### API 接口测试 (api_tests.rs)
- ✅ **test_get_recent_clips_command** - 获取最近剪贴板
- ✅ **test_search_clips_command** - 搜索剪贴板
- ✅ **test_api_pagination** - API分页
- ✅ **test_api_error_handling** - 错误处理
- ✅ **test_concurrent_access** - 并发访问
- ✅ **test_api_content_types** - 内容类型

### 颜色检测测试 (color_detection_tests.rs)
- ✅ **test_hex_3_digit** - HEX 3位 (#RGB)
- ✅ **test_hex_6_digit** - HEX 6位 (#RRGGBB)
- ✅ **test_hex_8_digit** - HEX 8位 (#RRGGBBAA)
- ✅ **test_rgb_format** - RGB 格式
- ✅ **test_rgba_format** - RGBA 格式
- ✅ **test_hsl_format** - HSL 格式
- ✅ **test_hsla_format** - HSLA 格式
- ✅ **test_invalid_hex** - 无效HEX
- ✅ **test_invalid_rgb** - 无效RGB
- ✅ **test_whitespace_handling** - 空白处理
- ✅ **test_case_insensitive** - 大小写不敏感

## 运行测试

### 运行所有测试
```bash
cd src-tauri
cargo test
```

### 运行特定测试文件
```bash
cargo test --test db_tests
cargo test --test api_tests
cargo test --test color_detection_tests
```

### 运行特定测试
```bash
cargo test test_add_color
cargo test test_hex_3_digit
```

### 带输出的测试
```bash
cargo test -- --nocapture
```

### 安静模式
```bash
cargo test --quiet
```

## 测试结果

最新测试运行结果：
- **总测试数**: 28
- **通过**: 28
- **失败**: 0
- **忽略**: 0

```
running 6 tests (api_tests)
test result: ok. 6 passed

running 11 tests (db_tests)
test result: ok. 11 passed

running 11 tests (color_detection_tests)
test result: ok. 11 passed
```

## 公共测试工具 (common/mod.rs)

### 辅助函数
- `create_test_dir()` - 创建临时测试目录
- `get_test_data_dir()` - 获取测试数据目录路径
- `test_color_samples()` - 颜色值测试样本
- `test_non_color_samples()` - 非颜色值测试样本
- `cleanup_test_db()` - 清理测试数据库

## 测试要点

### 1. 使用临时目录
所有测试使用 `tempfile::TempDir` 创建隔离的测试环境，测试结束后自动清理。

### 2. 颜色检测覆盖
支持7种颜色格式的完整测试：
- HEX: #RGB, #RRGGBB, #RRGGBBAA
- RGB/RGBA: rgb(), rgba()
- HSL/HSLA: hsl(), hsla()

### 3. 并发测试
`test_concurrent_access` 验证数据库在多线程环境下的安全性。

### 4. 时间戳处理
测试代码避免依赖于 `created_at` 的精确顺序，因为连续插入可能产生相同的时间戳。

### 5. 标签数组
验证 tags 字段正确存储为 JSON 数组，支持多标签。

## 待扩展测试

未来可以添加的测试：
- [ ] 图片文件处理测试
- [ ] 文件路径处理测试
- [ ] 置顶功能测试
- [ ] FTS 全文搜索性能测试
- [ ] Hook 事件系统测试（待实现）
- [ ] 前端集成测试
- [ ] 跨平台路径测试

## 注意事项

1. 某些 warning（如 unused imports）可以通过 `cargo fix` 修复
2. 测试使用 SQLite 内存模式可以提升速度（未来优化）
3. 颜色检测正则表达式已通过200+样本验证
4. 所有测试相互独立，可以并行运行
