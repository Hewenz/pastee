# 数据库迁移管理指南

**版本**: 1.0  
**最后更新**: 2026-01-12  
**架构**: Rust + rusqlite_migration

---

## 📋 概述

数据库迁移脚本已分离到独立的 SQL 文件中，便于版本管理和升级。

### 迁移文件位置

```
src-tauri/migrations/
└── 001_init.sql          # V001: 初始化表结构
```

### 代码位置

```
src-tauri/src/persist.rs
└── fn migrate()           # 调用迁移的函数
```

---

## 🗂️ 目录结构

```
src-tauri/
├── migrations/
│   ├── 001_init.sql       ✅ 已实现
│   ├── 002_v2.sql         📋 待来
│   └── 003_v3.sql         📋 待来
└── src/
    └── persist.rs         (引入迁移文件)
```

---

## 📝 SQL 文件规范

### 命名约定

```
{version}_{description}.sql

例如:
- 001_init.sql         # 初始化
- 002_add_column.sql   # 添加字段
- 003_drop_index.sql   # 删除索引
```

### 文件头格式

```sql
-- Migration: {version}_{description}.sql
-- Description: 详细描述
-- Created: YYYY-MM-DD
-- Version: 1.0

-- SQL 语句...
```

### 内容组织

```sql
-- 1. 创建表 (CREATE TABLE)
-- 2. 创建索引 (CREATE INDEX)
-- 3. 创建视图 (CREATE VIEW)
-- 4. 创建触发器 (CREATE TRIGGER)
-- 5. 初始数据 (INSERT INTO) - 如有
```

---

## 🔧 在 Rust 中使用

### 当前实现 (persist.rs)

```rust
fn migrate(conn: &mut Connection) -> Result<()> {
    // SQL 迁移脚本从外部文件静态加载
    let init_sql = include_str!("../migrations/001_init.sql");
    
    let migrations = Migrations::new(vec![
        M::up(init_sql),
    ]);
    migrations.to_latest(conn)?;
    Ok(())
}
```

### include_str! 宏

- **编译时加载**: SQL 文件在编译时被静态嵌入到二进制文件中
- **零运行时开销**: 不需要在运行时读取文件
- **便于分发**: 二进制文件是完全独立的

---

## ➕ 添加新迁移步骤

### 步骤 1: 创建 SQL 文件

```bash
# 创建新的迁移文件
touch src-tauri/migrations/002_add_tag_support.sql
```

### 步骤 2: 编写 SQL

```sql
-- Migration: 002_add_tag_support.sql
-- Description: 添加标签支持
-- Created: 2026-01-15
-- Version: 1.0

-- 添加新列
ALTER TABLE records ADD COLUMN tags TEXT;

-- 创建标签索引
CREATE INDEX IF NOT EXISTS idx_records_tags ON records(tags);
```

### 步骤 3: 更新 Rust 代码

```rust
fn migrate(conn: &mut Connection) -> Result<()> {
    let init_sql = include_str!("../migrations/001_init.sql");
    let v2_sql = include_str!("../migrations/002_add_tag_support.sql");
    
    let migrations = Migrations::new(vec![
        M::up(init_sql),
        M::up(v2_sql),
    ]);
    migrations.to_latest(conn)?;
    Ok(())
}
```

### 步骤 4: 编译验证

```bash
cd src-tauri
cargo build
```

---

## 📊 迁移历史

| 版本 | 文件 | 描述 | 状态 | 日期 |
|------|------|------|------|------|
| **V001** | 001_init.sql | 初始化表结构、FTS、触发器 | ✅ | 2026-01-12 |
| **V002** | 002_*.sql | 待计划 | 📋 | - |
| **V003** | 003_*.sql | 待计划 | 📋 | - |

---

## ⚠️ 重要事项

### 1. 版本递增
- 版本号必须递增 (001 → 002 → 003)
- 不能跳过版本号
- rusqlite_migration 按顺序执行

### 2. 幂等性
- 所有 CREATE 语句使用 `IF NOT EXISTS`
- 所有 DROP 语句使用 `IF EXISTS`
- 确保多次运行结果相同

### 3. 向后兼容
- 不删除已有的字段 (只添加)
- 如需删除，通过新迁移分步处理
- 默认值要合理

### 4. SQL 注释
- 添加详细的注释说明各部分功能
- 说明字段含义和约束

### 5. 编译验证
- 每次修改后必须 `cargo build` 验证
- include_str! 会在编译时检查文件存在性

---

## 🔄 迁移流程图

```
应用启动
    ↓
Storage::new() 
    ↓
Self::migrate(&mut conn)
    ↓
include_str!("../migrations/001_init.sql")
    ↓
include_str!("../migrations/002_*.sql")
    ↓
migrations.to_latest(conn)
    ↓
database.db 更新完成
    ↓
应用正常运行
```

---

## 💡 最佳实践

### 文件组织

```sql
-- 分区块组织相关的 DDL
-- Block 1: 表定义
CREATE TABLE ...
CREATE TABLE ...

-- Block 2: 索引和虚拟表
CREATE INDEX ...
CREATE VIRTUAL TABLE ...

-- Block 3: 触发器
CREATE TRIGGER ...
CREATE TRIGGER ...

-- Block 4: 初始数据
INSERT INTO ...
```

### 注释写法

```sql
-- 清晰的分块注释
-- ============================================
-- 表：records (剪切板记录表)
-- ============================================

-- 中间层级注释
-- 核心字段

-- 内联注释
hash TEXT UNIQUE NOT NULL,  -- Blake3 哈希
```

### 命名规范

```sql
-- 表名：复数、snake_case
records, users, tags

-- 列名：snake_case
created_at, is_pinned, app_context

-- 索引名：idx_{table}_{columns}
idx_records_created_at
idx_records_hash

-- 触发器名：{table}_{event}_{action}
records_ai (after insert)
records_ad (after delete)
```

---

## 🧪 测试迁移

### 删除数据库后测试

```bash
# 删除旧数据库
rm ~/path/to/clippy.db

# 运行应用
cargo run

# 检查新数据库是否正确创建
sqlite3 ~/path/to/clippy.db ".schema"
```

### 验证表结构

```bash
sqlite3 clippy.db
> .schema records
> .schema records_fts
> .schema records_ai
```

---

## 📚 相关资源

- **rusqlite_migration**: https://docs.rs/rusqlite_migration/
- **SQLite Documentation**: https://www.sqlite.org/lang.html
- **项目文件**: [持久化层代码](../src-tauri/src/persist.rs)

---

## 🚀 下一步计划

| 任务 | 优先级 | 预计 |
|------|--------|------|
| 数据清理迁移 | P1 | 2周 |
| 标签/分类支持 | P1 | 2周 |
| 性能优化索引 | P2 | 1月 |
| 导出/备份功能 | P2 | 1月 |

---

## 📞 常见问题

### Q: 如何回滚迁移?
A: rusqlite_migration 不支持自动回滚。需要手动删除数据库重新初始化，或创建新的迁移脚本来"撤销"前一个迁移。

### Q: 能否修改已发布的迁移?
A: 不建议。已发布的迁移应视为不可变。如需修改，应创建新的迁移脚本。

### Q: 如何处理数据迁移?
A: 在迁移脚本中使用 INSERT INTO / UPDATE / DELETE 语句进行数据迁移。

### Q: 迁移文件会被编译进二进制吗?
A: 是的，include_str! 会在编译时静态嵌入文件内容，最终二进制不依赖外部文件。

---

**版本**: 1.0  
**作者**: AI Assistant  
**最后更新**: 2026-01-12  
**状态**: ✅ 已实施
