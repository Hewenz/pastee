# 数据目录配置 - 快速参考

## 📁 核心配置

**最后更新**: 2026-01-13

```
应用启动时的目录结构:
$HOME/Documents/pastee/
├── clippy.db           # SQLite 数据库
├── clippy.db-wal       # WAL 预写日志
├── clippy.db-shm       # 共享内存
└── images/             # 图片存储 (按月分片)
    ├── 202601/         # 年月 (YYYYMM)
    │   ├── original/   # 原图 (PNG)
    │   │   └── {timestamp}_{hash}.png
    │   └── thumbnail/  # 缩略图 (WebP 800x600 lossless)
    │       └── {timestamp}_{hash}.webp
    └── 202602/
        ├── original/
        └── thumbnail/
```

## 🖼️ 图片处理流程

1. **监听**: arboard 捕获 RGBA 数据 (width, height, rgba_data)
2. **去重**: Blake3 hash 计算，查询 image_hash 索引
3. **存储**:
   - 原图: RgbaImage → PNG 格式，保存到 YYYYMM/original/
   - 缩略图: resize(800x600) → WebP lossless，保存到 YYYYMM/thumbnail/
4. **Base64**: 缩略图编码为 base64，via emit 传输到前端
5. **数据库**: 记录 image_path, thumbnail_path, image_hash, width, height

## 🔧 代码位置

| 文件 | 修改内容 | 行号 |
|------|---------|------|
| `Cargo.toml` | 添加 `dirs = "5.0.1"` | 36 |
| `lib.rs` | `dirs::home_dir()` + `data_dir` 构造 | 45-51 |
| `persist.rs` | 无需修改 (已支持任意路径) | - |

## 💻 代码片段

### lib.rs 中的初始化

```rust
let home = dirs::home_dir().ok_or("Failed to get home directory")?;
let data_dir = home.join("Documents").join("pastee");
let storage = Storage::new(&data_dir).unwrap();
```

## ✅ 验证

**编译**: `cargo build`  
**结果**: ✅ 成功 (1.85s, 2 warnings)

## 🌍 跨平台路径

| 平台 | 路径 |
|------|------|
| macOS | `/Users/{username}/Documents/pastee` |
| Windows | `C:\Users\{username}\Documents\pastee` |
| Linux | `/home/{username}/Documents/pastee` |

## ⚠️ 注意

- `dirs` 库负责跨平台兼容性
- 目录会在应用启动时自动创建
- 数据库迁移在 `Storage::new()` 中自动执行
- WAL 模式启用，性能更优

## 📌 关键特性

✅ 用户可直接访问数据文件  
✅ 符合设计文档要求  
✅ 自动目录创建  
✅ 跨平台兼容  
✅ 0 个编译错误  

---

**状态**: 🟢 完成  
**验证**: ✅ cargo build  
**就绪**: 📌 可进行前端对接
