# 后端方案3 - 快速参考卡

**完成日期**: 2026-01-12  
**编译状态**: ✅ 通过  
**下一步**: 前端对接

---

## 🔧 核心改动速查

### 1️⃣ 依赖更新
```toml
# 删除
- sha2 = "0.10.9"

# 新增  
+ blake3 = "1.5.0"
```

### 2️⃣ 数据库
| 项 | 旧 | 新 |
|---|---|---|
| **表名** | clips | records |
| **FTS表** | clips_fts | records_fts |
| **字段** | content_hash | hash |
| **DB文件** | history.db | clippy.db |
| **新字段** | - | app_context |

### 3️⃣ Hash 函数
```rust
// 改为
fn compute_hash(data: &[u8]) -> String {
    let hash = blake3::hash(data);
    hex::encode(hash.as_bytes())
}
```

### 4️⃣ SQL 查询
```sql
-- 所有 FROM clips 改为 FROM records
-- 所有 clips_fts 改为 records_fts
-- 所有 content_hash 改为 hash
```

### 5️⃣ IPC
```rust
// 删除
- select_clip_item()

// 保留
- get_recent_clips(limit, offset)
- search_clips(query)
```

---

## 📊 文件改动清单

| 文件 | 改动 | 完成 |
|-----|------|------|
| `Cargo.toml` | 依赖替换 | ✅ |
| `clipboard.rs` | Hash 算法 | ✅ |
| `persist.rs` | Schema + SQL | ✅ |
| `lib.rs` | IPC 接口 | ✅ |

---

## 🎯 MVP 功能状态

**最后更新**: 2026-01-13

| 功能 | 状态 | 备注 |
|------|------|------|
| **监听** | ✅ | 基于 arboard，支持文本/HTML/图片 |
| **存储** | ✅ | records 表，图片按月分片 |
| **分页** | ✅ | get_recent_clips |
| **搜索** | ✅ | search_clips (LIKE) |
| **去重** | ✅ | Blake3 hash |
| **推送** | ✅ | Event emit (clipboard://new-clip) |
| **置顶** | ✅ | toggle_pin |
| **删除** | ✅ | delete_clip |
| **清空** | ✅ | clear_unpinned_clips |
| **图片处理** | ✅ | RGBA→PNG+WebP缩略图 |
| **异步处理** | ✅ | 三事件系统 |
| **总计数** | ✅ | get_total_count |
| **窗口保持** | ✅ | set_keep_window_open |
| **快捷键** | ✅ | Cmd+Shift+V (macOS) |
| **托盘** | ✅ | 系统托盘集成 |
| **粘贴** | ⏳ | Auto-Paste P0 功能 |
| **黑名单** | ⏳ | 隐私App检测 |

---

## 🔌 API 接口

### Commands

```rust
// 分页查询
get_recent_clips(limit: usize, offset: usize) -> Vec<ClipItem>

// 模糊搜索  
search_clips(query: String) -> Vec<ClipItem>

// 获取总计数
get_total_count() -> i64

// 清空未置顶记录
clear_unpinned_clips() -> i64

// 置顶/取消置顶
toggle_pin(id: i64) -> bool

// 删除记录
delete_clip(id: i64) -> i64

// 窗口控制
toggle_window()
set_keep_window_open(keep: bool)

// 图片URL获取
get_image_url(id: i64, thumbnail: bool) -> String
```

### Events

```rust
// 新记录推送
clipboard://new-clip -> ClipItem

// 图片处理中
clipboard://image-pending -> { temp_id: number }

// 图片处理完成
clipboard://image-ready -> { temp_id: number, id: i64, thumbnail: String }
```

### Data Structures

```rust
pub struct ClipItem {
    pub id: i64,
    pub content_type: ClipType,  // Text | Html | Color | Image | Files
    pub preview: String,
    pub created_at: i64,
    pub is_pinned: bool,
}
```

---

## ⚠️ 编译警告 (暂不处理)

```
unused import: DateTime
unused import: Emitter
unused variable: files_json
unused variable: tray
unused variable: app_handle
```

**说明**: P0 功能实现时会使用这些，暂留。

---

## 🚀 下一步行动

### 即刻开始 (前端)
1. [ ] 启用真实 API 调用
2. [ ] 移除 mockData
3. [ ] 集成虚拟滚动
4. [ ] Zustand 状态管理

### 之后 (P0)
5. [ ] 自动粘贴
6. [ ] 全局快捷键
7. [ ] Event 推送

---

## 💾 数据库路径

```
$HOME/AppData/Local/com.kylin.pastee/  (Windows)
  └── clippy.db
  └── images/
      └── {hash}.png

~/Library/Application Support/com.kylin.pastee/  (macOS)
  └── clippy.db
  └── images/
      └── {hash}.png
```

---

## 📌 关键链接

- 详细实现: [backend_implementation_complete.md](backend_implementation_complete.md)
- 完整对比: [design_implementation_diff.md](design_implementation_diff.md)
- 依赖对比: [dependencies_detailed_comparison.md](dependencies_detailed_comparison.md)

---

## ✨ 提示

- Blake3 哈希输出 64 字符，与 SHA256 兼容
- 现有索引无需修改
- 可直接启动应用，数据库会自动建表
- 旧的 history.db 可删除

---

**状态**: ✅ 完成  
**编译**: ✅ 通过  
**下一步**: 前端对接 👉
