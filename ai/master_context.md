# Project Context: ClippyTauri (Open Source Clipboard Manager)

> **Role Instruction for AI**: 
> 你是 "虚拟软件研发团队"。当前处于 **Phase 4: 开发与实现** 阶段。
> 请读取以下所有项目上下文、架构设计和进度状态，继续执行开发任务。
> 严禁修改已冻结的 P0 需求，除非用户明确要求变更。

## 1. 🎯 项目概览 (Project Profile)
* **项目名称**: ClippyTauri (暂定)
* **核心定位**: 免费、开源、跨平台 (Win/Mac)、隐私优先 (Local-First) 的剪切板管理工具。
* **对标竞品**: Raycast (Win版), Maccy, Paste。
* **差异化**: 比 Electron 轻量 (Rust+Tauri)，比原生 Win+V 强大，完全离线。
* **目标平台**: 
    * **Tier 0 (支持)**: Windows 10/11, macOS (Ventura+).
    * **Unsupported**: Linux (仅提供源码，不保证兼容性).

## 2. 📝 需求规格 (PRD - Frozen)

### 2.1 核心功能 (MVP P0)
1.  **监听 (Listening)**: 
    * 后端 Rust 实时监听系统剪切板。
    * **去重**: 连续复制相同内容仅更新时间戳。
    * **隐私黑名单**: 检测到特定 App (如 1Password, KeyChain) 前台运行时，自动暂停监听。
2.  **存储 (Storage)**:
    * **文本**: 存入 SQLite。
    * **图片**: 文件存入 `$HOME/Documents/ClippyData/images/`，数据库仅存路径。
    * **清理**: 启动时检查，保留最近 30 天或 N 条记录。
3.  **交互 (UI/UX)**:
    * **唤起**: 全局快捷键 (`Alt+V` / `Option+V`)。
    * **窗口**: 居中弹窗 (Spotlight 风格) 或 跟随鼠标 (配置可选)。
    * **粘贴行为**: 选中 -> 窗口隐藏 -> **自动模拟 `Ctrl+V`** 上屏 (Auto-Paste)。
4.  **检索 (Search)**:
    * 支持模糊搜索 (FTS5)。

### 2.2 技术栈 (Tech Stack)
* **Core**: Tauri 2.0 (Beta/RC)
* **Backend**: Rust
    * DB: `rusqlite` (Sync, bundled SQLite)
    * Image: `image` crate
    * Crypto: `blake3` (Hashing)
* **Frontend**: React + TypeScript + Vite
    * UI: ShadcnUI (Radix + Tailwind)
    * State: Zustand
    * List: `virtua` (Virtual Scrolling)

---

## 3. 🏗️ 系统架构与数据设计 (Architecture)

### 3.1 目录结构
```text
$HOME/Documents/ClippyData/
├── clippy.db            # SQLite (WAL Mode)
├── images/              # Image Store
│   ├── 2024/            # Year Sharding
│   │   ├── 01/          # Month Sharding
│   │   │   └── {hash}.png
└── logs/

```

### 3.2 数据库 Schema (SQLite)

*需包含 Migration 逻辑 (user_version check)*

```sql
-- Table: records
CREATE TABLE IF NOT EXISTS records (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    type        TEXT NOT NULL,       -- 'text' | 'image'
    content     TEXT,                -- Text content or OCR result
    data_path   TEXT,                -- Nullable, path to image file
    hash        TEXT UNIQUE NOT NULL,-- Blake3 Hash
    is_pinned   INTEGER DEFAULT 0,   -- 0 or 1
    created_at  INTEGER NOT NULL,    -- Unix Timestamp
    app_context TEXT                 -- Source App Name
);

-- Table: records_fts (Virtual Table)
CREATE VIRTUAL TABLE IF NOT EXISTS records_fts USING fts5(
    content, content='records', content_rowid='id'
);

-- Triggers: Sync records -> records_fts (Insert/Delete/Update)
-- [Code omitted for brevity, verify in implementation]

```

### 3.3 接口协议 (IPC Contract)

采用 **Push + Pull 混合模式**：

1. **Pull (分页拉取)**:
* **Command**: `get_records(limit: u32, offset: u32, search: Option<String>)`
* **Logic**:
* 若 `search` 为空 -> `ORDER BY created_at DESC LIMIT n OFFSET m`
* 若 `search` 有值 -> FTS5 Match -> Return Results




2. **Push (实时推送)**:
* **Event**: `clipboard://new-record`
* **Payload**: `ClipboardRecord` (JSON)
* **Trigger**: 当 Rust 监听到复制并成功写入 DB 后触发，前端接收后直接 unshift 到列表头部。



### 3.4 Rust 数据结构

```rust
#[derive(Serialize, Deserialize)]
pub struct ClipboardRecord {
    pub id: i64,
    pub r#type: String, // "text" or "image"
    pub content: String,
    pub data_path: Option<String>,
    pub is_pinned: bool,
    pub created_at: i64,
}

```

---

## 4. 📅 进度跟踪 (Project State)

**当前阶段**: Phase 4 - 开发与实现 (进行中)
**最后更新**: 2026-01-13

### ✅ 已完成 (Done)

* [x] Phase 1: 需求冻结 (放弃 Linux 支持，确定 P0 功能)。
* [x] Phase 2: 技术选型 (Rust/Tauri2/SQLite/React)。
* [x] Phase 3: 详细设计 (Schema, IPC 签名, 存储策略)。
* [x] **数据库迁移合并**: 所有迁移文件合并为 001_schema_init.sql
* [x] **图片存储系统**: RGBA数据处理、PNG原图、WebP缩略图、Blake3去重
* [x] **异步图片处理**: 三事件系统 (image-pending → 处理 → image-ready)
* [x] **Base64缩略图流式传输**: 直接emit base64编码的缩略图数据
* [x] **UI优化**: 状态栏、暗黑模式、OneHalf Light配色、分页居中
* [x] **一键清空**: clear_unpinned_clips 命令，保留置顶数据
* [x] **窗口保持打开**: 控制失焦时是否自动隐藏
* [x] **全局快捷键**: Cmd+Shift+V (唤起窗口)
* [x] **托盘图标**: 系统托盘集成

### ⏳ 待办事项 (Todo List)

> **AI 请注意：这是你的任务清单，请按顺序执行。**

#### Step 1: 基础脚手架

* [x] 初始化 Tauri 2.0 项目结构。
* [x] 配置 `tauri.conf.json` (Capabilities, Permissions: `fs`, `clipboard`).
* [x] 安装 Rust 依赖 (`rusqlite`, `serde`, `image`, `blake3`).

#### Step 2: 数据库层 (Rust)

* [x] 实现数据库初始化与 Migration 逻辑。
* [x] 实现 `insert_record` (带去重) 和 `query_records` (带 FTS).
* [x] 实现图片存储逻辑 (按月分片: YYYYMM/original, YYYYMM/thumbnail).

#### Step 3: 监听与业务层 (Rust)

* [x] 实现剪切板监听线程 (基于 arboard).
* [x] 实现图片RGBA数据处理。
* [x] 实现HTML文本提取和颜色检测。
* [ ] 实现 `Auto-Paste` 逻辑 (焦点控制 + 模拟按键).
* [ ] 实现隐私黑名单 (特定App禁用监听).

#### Step 4: 前端对接 (React)

* [x] 搭建 UI 框架 (ShadcnUI).
* [x] 实现基本列表显示。
* [x] 对接 IPC `get_recent_clips` 和 Event `clipboard://new-clip`.
* [x] 实现搜索功能 (search_clips).
* [x] 实现置顶/删除功能。
* [x] 实现暗黑模式切换。
* [ ] 实现虚拟列表 (`virtua`) - 性能优化。

---

## 5. ⚠️ 关键注意事项 (Critical Notes)

1. **macOS 权限**: 访问 `$Home/Documents` 需要处理 Sandbox 或权限请求，若失败需优雅降级或提示。
2. **自动粘贴**: Windows 下 `SetForegroundWindow` 后需要微小的 `sleep` 才能发送 `Ctrl+V`，否则会粘贴失败。
3. **No Network**: 严禁在代码中引入任何非必要的网络请求。
