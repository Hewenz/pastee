-- Migration: 001_schema_init.sql
-- Description: 初始化剪切板管理系统的完整数据库 schema
-- Created: 2026-01-13
-- Version: 1.0
--
-- 包含：
-- - 剪切板记录表 (支持多种内容类型)
-- - 标签字段 (支持多标签分类)
-- - 图片相关字段 (路径、缩略图、元数据)
-- - FTS 全文搜索索引
-- - 自动同步触发器

-- ============================================================================
-- 表：records - 剪切板记录主表
-- ============================================================================
CREATE TABLE IF NOT EXISTS records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,
    
    -- 核心字段，不同类型存不同列
    content_text TEXT,       -- 纯文本 / HTML的纯文本部分 / 文件列表的拼接字符串
    content_html TEXT,       -- HTML 原始内容
    content_image_path TEXT, -- 图片文件名
    content_file_paths TEXT, -- JSON 格式的文件路径数组
    
    hash TEXT UNIQUE NOT NULL,        -- Blake3 哈希指纹 (用于去重)
    created_at INTEGER NOT NULL,      -- 创建时间戳 (Unix timestamp, 微秒级精度)
    is_pinned BOOLEAN DEFAULT 0,      -- 是否固定
    tag TEXT DEFAULT '["text"]',      -- JSON 格式的标签数组，用于快速分类
    app_context TEXT,                 -- 源应用名称 (用于隐私黑名单)
    
    -- 图片相关字段
    image_path TEXT,         -- 图片原始文件相对路径
    thumbnail_path TEXT,     -- 缩略图文件相对路径
    image_format TEXT,       -- 图片格式 (png, jpg, webp 等)
    image_size INTEGER,      -- 图片文件大小 (字节)
    image_hash TEXT,         -- 图片内容哈希 (用于去重)
    width INTEGER,           -- 图片宽度
    height INTEGER           -- 图片高度
);

-- ============================================================================
-- 索引：加速查询和去重
-- ============================================================================
CREATE INDEX IF NOT EXISTS idx_records_tag ON records(tag);
CREATE INDEX IF NOT EXISTS idx_image_hash ON records(image_hash);


-- ============================================================================
-- 虚拟表：records_fts - FTS5 全文搜索索引
-- ============================================================================
-- 用于快速模糊搜索剪切板内容
CREATE VIRTUAL TABLE IF NOT EXISTS records_fts USING fts5(
    content_text,
    tag,
    content='records',
    content_rowid='id'
);

-- ============================================================================
-- 触发器：自动同步 FTS 索引
-- ============================================================================

-- 插入触发器：新插入的记录自动同步到 FTS
CREATE TRIGGER IF NOT EXISTS records_ai AFTER INSERT ON records BEGIN
    INSERT INTO records_fts(rowid, content_text, tag)
    VALUES (new.id, new.content_text, new.tag);
END;

-- 删除触发器：删除的记录自动从 FTS 移除
CREATE TRIGGER IF NOT EXISTS records_ad AFTER DELETE ON records BEGIN
    INSERT INTO records_fts(records_fts, rowid, content_text, tag)
    VALUES ('delete', old.id, old.content_text, old.tag);
END;

-- 更新触发器：更新的记录自动同步到 FTS
CREATE TRIGGER IF NOT EXISTS records_au AFTER UPDATE ON records BEGIN
    INSERT INTO records_fts(records_fts, rowid, content_text, tag)
    VALUES ('delete', old.id, old.content_text, old.tag);
    INSERT INTO records_fts(rowid, content_text, tag)
    VALUES (new.id, new.content_text, new.tag);
END;
