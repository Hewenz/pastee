use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use rusqlite_migration::{Migrations, M};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use image::GenericImageView;



#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ClipType {
    Text,
    Image,
    Html,
    Files,
    Color,
}
impl ToString for ClipType {
    fn to_string(&self) -> String {
        match self {
            ClipType::Text => "text".to_string(),
            ClipType::Html => "html".to_string(),
            ClipType::Image => "image".to_string(),
            ClipType::Files => "files".to_string(),
            ClipType::Color => "color".to_string(),
        }
    }
}

impl From<String> for ClipType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "html" => ClipType::Html,
            "image" => ClipType::Image,
            "files" => ClipType::Files,
            "color" => ClipType::Color,
            _ => ClipType::Text,
        }
    }
}

/// 列表项（轻量级，用于 UI 展示）
#[derive(Debug, Serialize, Deserialize)]
pub struct ClipItem {
    pub id: i64,
    pub content_type: ClipType,
    pub preview: String,   // 预览文本
    pub created_at: i64,
    pub is_pinned: bool,
    pub tags: Vec<String>,  // 标签数组：["color", "favorite"], ["image", "work"] 等
}


#[derive(Debug, Serialize, Deserialize)]
pub enum ClipData {
    Text(String),
    Html { text: String, html: String }, // HTML 通常包含纯文本 fallback
    Image(Vec<u8>),
    Files(Vec<String>), // 文件路径列表
    Color(String),      // 颜色值（保存原始格式）
}

pub struct Storage {
    conn: Connection,
    image_dir: PathBuf,
}

impl Storage {
    pub fn new<P: AsRef<Path>>(data_dir: P) -> Result<Self> {
        let data_dir = data_dir.as_ref().to_path_buf();
        let image_dir = data_dir.join("images");
        let db_path = data_dir.join("clippy.db");

        if !image_dir.exists() {
            fs::create_dir_all(&image_dir).context("Failed to create image dir")?;
        }

        let mut conn = Connection::open(&db_path).context("Failed to open DB")?;
        
        // 性能调优
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;

        Self::migrate(&mut conn)?;

        Ok(Self { conn, image_dir })
    }

    fn migrate(conn: &mut Connection) -> Result<()> {
        // SQL 迁移脚本从外部文件 migrations/*.sql 静态加载
        let schema_sql = include_str!("../migrations/001_schema_init.sql");
        
        let migrations = Migrations::new(vec![
            M::up(schema_sql),
        ]);
        migrations.to_latest(conn)?;
        Ok(())
    }


    /// 1. 存纯文本
    pub fn add_text(&mut self, text: String) -> Result<i64> {
        let text = text.trim().to_string();
        if text.is_empty() { return Ok(0); }
        let hash = Self::compute_hash(text.as_bytes());

        // 检测是否为颜色值，设置 tags 数组
        let (clip_type, tags) = if Self::is_color(&text) {
            (ClipType::Color, vec!["color".to_string()])
        } else {
            (ClipType::Text, vec!["text".to_string()])
        };

        let tx = self.conn.transaction()?;
        let id = Self::upsert_record(&tx, clip_type, &hash, &tags, |sql, params| {
             tx.execute(sql, params)
        }, Some(&text), None, None, None)?;
        tx.commit()?;
        Ok(id)
    }

    /// 2. 存 HTML (同时存纯文本用于搜索)
    pub fn add_html(&mut self, text_preview: String, html_content: String) -> Result<i64> {
        // 检测 text_preview 是否为颜色值，如果是则保存为 Color 类型
        let text_trimmed = text_preview.trim();
        if Self::is_color(text_trimmed) {
            // 直接保存为颜色
            return self.add_text(text_trimmed.to_string());
        }
        
        // HTML 的指纹计算：建议用 html 内容算，或者 text+html 混合算
        let hash = Self::compute_hash(html_content.as_bytes());
        
        let tx = self.conn.transaction()?;
        let id = Self::upsert_record(&tx, ClipType::Html, &hash, &vec!["html".to_string()], |sql, params| {
             tx.execute(sql, params)
        }, Some(&text_preview), Some(&html_content), None, None)?;
        tx.commit()?;
        Ok(id)
    }

    /// 3. 存图片 (已被新的add_image方法替代，此方法已删除)

    /// 4. 存文件路径列表 (Vec<Path>)
    pub fn add_files(&mut self, paths: Vec<String>) -> Result<i64> {
        if paths.is_empty() { return Ok(0); }
        
        // 序列化为 JSON 存入 DB
        let json_str = serde_json::to_string(&paths)?;
        // 将所有文件名拼接成字符串，用于全文搜索
        // 比如: "C:\Users\Photo.jpg" -> 存入 content_text 以便能搜到 "Photo"
        let search_text = paths.join("\n"); 
        
        let hash = Self::compute_hash(json_str.as_bytes());

        let tx = self.conn.transaction()?;
        let id = Self::upsert_record(&tx, ClipType::Files, &hash, &vec!["files".to_string()], |sql, params| {
             tx.execute(sql, params)
        }, Some(&search_text), None, None, Some(&json_str))?;
        tx.commit()?;
        Ok(id)
    }

    /// 获取列表
    pub fn get_recent(&self, limit: usize, offset: usize) -> Result<Vec<ClipItem>> {
        println!("🔍 查询最近记录: limit={}, offset={}", limit, offset);
        
        let mut stmt = self.conn.prepare(
            "SELECT id, type, content_text, content_file_paths, created_at, is_pinned, tag,
             image_format, width, height
             FROM records 
             ORDER BY is_pinned DESC, created_at DESC 
             LIMIT ?1 OFFSET ?2"
        )?;

        let rows = stmt.query_map(params![limit, offset], |row| {
            let id: i64 = row.get(0)?;
            let type_str: String = row.get(1)?;
            let text: Option<String> = row.get(2)?;
            let files_json: Option<String> = row.get(3)?;
            let created_at: i64 = row.get(4)?;
            let is_pinned: bool = row.get(5)?;
            let tags_json: Option<String> = row.get(6)?;
            let image_format: Option<String> = row.get(7)?;
            let width: Option<i64> = row.get(8)?;
            let height: Option<i64> = row.get(9)?;

            let content_type = ClipType::from(type_str);
            
            // 解析 tags JSON 数组
            let tags = if let Some(json) = tags_json {
                serde_json::from_str::<Vec<String>>(&json).unwrap_or_else(|_| vec!["text".to_string()])
            } else {
                vec!["text".to_string()]
            };
            
            // 生成 UI 预览文字
            let preview = match content_type {
                ClipType::Text | ClipType::Html => {
                    text.unwrap_or_default().chars().take(100).collect::<String>().replace('\n', " ")
                },
                ClipType::Color => {
                    // 颜色直接显示值
                    text.unwrap_or_default()
                },
                ClipType::Image => {
                    // 显示图片信息
                    if let (Some(w), Some(h), Some(fmt)) = (width, height, image_format) {
                        format!("[图片] {}x{} {}", w, h, fmt.to_uppercase())
                    } else {
                        "[图片]".to_string()
                    }
                },
                ClipType::Files => {
                    // 尝试解析 JSON 看看有几个文件
                    if let Some(json) = files_json {
                        if let Ok(paths) = serde_json::from_str::<Vec<String>>(&json) {
                            format!("[文件] {} 个项目: {}", paths.len(), paths.first().unwrap_or(&"".to_string()))
                        } else {
                            "[文件列表]".to_string()
                        }
                    } else {
                        "[文件列表]".to_string()
                    }
                }
            };

            Ok(ClipItem {
                id,
                content_type,
                preview,
                created_at,
                is_pinned,
                tags,
            })
        })?;

        let mut items = Vec::new();
        for row in rows { items.push(row?); }
        println!("✅ get_recent 查询到 {} 条记录", items.len());
        Ok(items)
    }

    /// 获取总记录数
    pub fn get_total_count(&self) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM records",
            [],
            |row| row.get(0)
        )?;
        println!("📊 数据库总记录数: {}", count);
        Ok(count)
    }

    /// 搜索 (所有类型都通过 content_text 搜索)
    pub fn search(&self, query: &str) -> Result<Vec<ClipItem>> {
        // 使用 LIKE 查询支持中文和模糊匹配
        let like_query = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));
        let mut stmt = self.conn.prepare(
            "SELECT id, type, content_text, content_file_paths, created_at, is_pinned, tag 
             FROM records 
             WHERE content_text LIKE ?1 ESCAPE '\\'
             ORDER BY created_at DESC LIMIT 50"
        )?;
        
        let rows = stmt.query_map(params![like_query], |row| {
             // 复制上面的 row mapping 逻辑
             let id: i64 = row.get(0)?;
             let type_str: String = row.get(1)?;
             let text: Option<String> = row.get(2)?;
             let _files_json: Option<String> = row.get(3)?;
             let created_at: i64 = row.get(4)?;
             let is_pinned: bool = row.get(5)?;
             let tags_json: Option<String> = row.get(6)?;
             let content_type = ClipType::from(type_str);
             
             // 解析 tags JSON 数组
             let tags = if let Some(json) = tags_json {
                 serde_json::from_str::<Vec<String>>(&json).unwrap_or_else(|_| vec!["text".to_string()])
             } else {
                 vec!["text".to_string()]
             };
             
             let preview = match content_type {
                ClipType::Text | ClipType::Html => text.unwrap_or_default().chars().take(50).collect(),
                ClipType::Color => text.unwrap_or_default(),
                ClipType::Image => "[图片]".to_string(),
                ClipType::Files => "[文件]".to_string(),
            };
            Ok(ClipItem { id, content_type, preview, created_at, is_pinned, tags })
        })?;

        let mut items = Vec::new();
        for row in rows { items.push(row?); }
        Ok(items)
    }

    /// 获取详情 (用于粘贴)
    pub fn get_content(&self, id: i64) -> Result<ClipData> {
        let mut stmt = self.conn.prepare(
            "SELECT type, content_text, content_html, content_image_path, content_file_paths,
             image_path, thumbnail_path
             FROM records WHERE id = ?1"
        )?;
        
        let item = stmt.query_row(params![id], |row| {
            let type_str: String = row.get(0)?;
            let text: Option<String> = row.get(1)?;
            let html: Option<String> = row.get(2)?;
            let img_path_old: Option<String> = row.get(3)?;
            let file_paths: Option<String> = row.get(4)?;
            let image_path: Option<String> = row.get(5)?;
            let _thumbnail_path: Option<String> = row.get(6)?;
            
            Ok((type_str, text, html, img_path_old, file_paths, image_path))
        })?;

        let (t_str, text, html, img_path_old, file_paths, image_path) = item;

        match ClipType::from(t_str) {
            ClipType::Text => Ok(ClipData::Text(text.unwrap_or_default())),
            ClipType::Color => Ok(ClipData::Color(text.unwrap_or_default())),
            ClipType::Html => Ok(ClipData::Html {
                text: text.unwrap_or_default(),
                html: html.unwrap_or_default(),
            }),
            ClipType::Image => {
                // 优先使用新字段 image_path，兼容旧数据
                let path = image_path.or(img_path_old)
                    .ok_or_else(|| anyhow::anyhow!("Image path not found"))?;
                let full_path = self.image_dir.join(path);
                let bytes = fs::read(full_path)?;
                Ok(ClipData::Image(bytes))
            },
            ClipType::Files => {
                if let Some(json) = file_paths {
                    let paths: Vec<String> = serde_json::from_str(&json).unwrap_or_default();
                    Ok(ClipData::Files(paths))
                } else {
                    Ok(ClipData::Files(vec![]))
                }
            }
        }
    }

    /// 获取图片的缩略图路径（用于前端展示）
    pub fn get_image_paths(&self, id: i64) -> Result<(String, String)> {
        self.conn.query_row(
            "SELECT image_path, thumbnail_path FROM records WHERE id = ?1",
            params![id],
            |row| {
                let image_path: String = row.get(0)?;
                let thumbnail_path: String = row.get(1)?;
                Ok((image_path, thumbnail_path))
            },
        )
        .context("Failed to get image paths")
    }

    // ==========================================
    // 内部 helper
    // ==========================================

    /// 通用的 Upsert 逻辑
    fn upsert_record<F>(
        tx: &Transaction,
        ctype: ClipType,
        hash: &str,
        tags: &[String],
        executor: F, // 闭包，用于执行具体的 SQL
        
        // 各种可选字段
        text: Option<&str>,
        html: Option<&str>,
        img_path: Option<&str>,
        file_paths: Option<&str>,
    ) -> Result<i64>
    where
        F: FnOnce(&str, &[&dyn rusqlite::ToSql]) -> rusqlite::Result<usize>,
    {
        // 1. 将 tags 数组序列化为 JSON
        let tags_json = serde_json::to_string(tags)?;
        
        // 2. 构造 SQL
        let sql = "INSERT INTO records (type, hash, created_at, content_text, content_html, content_image_path, content_file_paths, tag)
                   VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                   ON CONFLICT(hash) DO UPDATE SET
                      created_at = excluded.created_at,
                      tag = excluded.tag";
        
        // 3. 执行
        executor(sql, params![
            ctype.to_string(),
            hash,
            Utc::now().timestamp_micros(),
            text,
            html,
            img_path,
            file_paths,
            tags_json
        ])?;

        // 4. 获取 ID
        let id: i64 = tx.query_row(
            "SELECT id FROM records WHERE hash = ?1",
            params![hash],
            |row| row.get(0),
        )?;

        Ok(id)
    }

    fn find_id_by_hash(&self, hash: &str) -> Result<Option<i64>> {
        self.conn.query_row(
            "SELECT id FROM records WHERE hash = ?1",
            params![hash],
            |row| row.get(0),
        ).optional().map_err(Into::into)
    }
    
    fn touch_record(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE records SET created_at = ?1 WHERE id = ?2",
            params![Utc::now().timestamp_micros(), id],
        )?;
        Ok(())
    }

    fn compute_hash(data: &[u8]) -> String {
        let hash = blake3::hash(data);
        hex::encode(hash.as_bytes())
    }

    /// 检测字符串是否为颜色值
    /// 支持格式：
    /// - HEX: #RGB, #RRGGBB, #RRGGBBAA
    /// - RGB: rgb(r, g, b)
    /// - RGBA: rgba(r, g, b, a)
    /// - HSL: hsl(h, s%, l%)
    /// - HSLA: hsla(h, s%, l%, a)
    fn is_color(text: &str) -> bool {
        let text = text.trim();
        
        // HEX 格式: #RGB, #RRGGBB, #RRGGBBAA
        if text.starts_with('#') {
            let hex_part = &text[1..];
            let len = hex_part.len();
            // 验证长度和字符
            if (len == 3 || len == 6 || len == 8) && hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
                return true;
            }
        }
        
        // RGB/RGBA 格式
        let lower = text.to_lowercase();
        if lower.starts_with("rgb(") || lower.starts_with("rgba(") {
            if let (Some(start), Some(end)) = (lower.find('('), lower.rfind(')')) {
                let content = &lower[start+1..end];
                let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
                // rgb 需要 3 个参数，rgba 需要 4 个参数
                if (lower.starts_with("rgb(") && parts.len() == 3) || 
                   (lower.starts_with("rgba(") && parts.len() == 4) {
                    return true;
                }
            }
        }
        
        // HSL/HSLA 格式
        if lower.starts_with("hsl(") || lower.starts_with("hsla(") {
            if let (Some(start), Some(end)) = (lower.find('('), lower.rfind(')')) {
                let content = &lower[start+1..end];
                let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
                // hsl 需要 3 个参数，hsla 需要 4 个参数
                if (lower.starts_with("hsl(") && parts.len() == 3) || 
                   (lower.starts_with("hsla(") && parts.len() == 4) {
                    return true;
                }
            }
        }
        
        false
    }

    /// 切换记录的置顶状态
    pub fn toggle_pin(&self, id: i64) -> Result<bool> {
        let new_state: bool = self.conn.query_row(
            "SELECT is_pinned FROM records WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        
        self.conn.execute(
            "UPDATE records SET is_pinned = ?1 WHERE id = ?2",
            params![!new_state, id],
        )?;
        
        Ok(!new_state)
    }

    /// 删除指定记录
    pub fn delete_record(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM records WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    /// 清空所有未置顶的记录
    pub fn clear_unpinned(&mut self) -> Result<i64> {
        let deleted = self.conn.execute("DELETE FROM records WHERE is_pinned = 0", [])?;
        println!("🗑️ 已清空 {} 条未置顶记录", deleted);
        Ok(deleted as i64)
    }

    /// 添加图片记录（Phase 1-3 实现）
    pub fn add_image(&mut self, width: usize, height: usize, rgba_data: Vec<u8>) -> Result<(i64, Vec<u8>)> {
        use image::{ImageFormat, RgbaImage};
        use std::time::{SystemTime, UNIX_EPOCH};

        println!("📸 开始处理图片: {}x{}, {} bytes", width, height, rgba_data.len());

        // 验证数据大小
        if width * height * 4 != rgba_data.len() {
            return Err(anyhow::anyhow!("图片数据大小不匹配: 期望 {} bytes, 实际 {} bytes", 
                width * height * 4, rgba_data.len()));
        }

        // 计算图片 hash（用于去重）
        let hash = blake3::hash(&rgba_data);
        let hash_hex = hex::encode(&hash.as_bytes()[..8]); // 取前8字节
        println!("📸 图片hash: {}", hash_hex);

        // Phase 3: 去重检查
        println!("📸 检查是否已存在...");
        match self.find_image_by_hash(&hash_hex) {
            Ok(Some(existing_id)) => {
                println!("📸 图片已存在，使用已有记录 ID: {}", existing_id);
                // 读取已存在的缩略图数据返回
                let (_, thumbnail_path) = self.get_image_paths(existing_id)?;
                let full_thumb_path = self.image_dir.join(&thumbnail_path);
                let thumbnail_data = fs::read(full_thumb_path)?;
                return Ok((existing_id, thumbnail_data));
            }
            Ok(None) => {
                println!("📸 图片不存在，继续保存");
            }
            Err(e) => {
                eprintln!("❌ 去重检查失败: {:?}", e);
                return Err(e);
            }
        }

        // 从 RGBA 原始数据创建图片
        let rgba_image = RgbaImage::from_raw(width as u32, height as u32, rgba_data.clone())
            .ok_or_else(|| anyhow::anyhow!("无法从 RGBA 数据创建图片"))?;
        let img = image::DynamicImage::ImageRgba8(rgba_image);

        let format = ImageFormat::Png; // 统一保存为 PNG
        let ext = "png";

        // 生成文件路径
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_micros();
        let year_month = chrono::Local::now().format("%Y%m").to_string();
        
        // 创建目录: images/202601/original 和 images/202601/thumbnail
        let month_dir = self.image_dir.join(&year_month);
        let original_dir = month_dir.join("original");
        let thumbnail_dir = month_dir.join("thumbnail");
        
        fs::create_dir_all(&original_dir)?;
        fs::create_dir_all(&thumbnail_dir)?;

        // 文件名: {timestamp}_{hash}.{ext}
        let filename = format!("{}_{}.{}", now, hash_hex, ext);
        let thumb_filename = format!("{}_{}.webp", now, hash_hex);
        
        // 相对路径（存储到DB）
        let relative_path = format!("{}/original/{}", year_month, filename);
        let relative_thumb_path = format!("{}/thumbnail/{}", year_month, thumb_filename);

        // 绝对路径（文件系统操作）
        let original_path = original_dir.join(&filename);
        let thumbnail_path = thumbnail_dir.join(&thumb_filename);

        // Phase 1: 保存原图（PNG格式）
        img.save_with_format(&original_path, format)
            .context("Failed to write original image")?;
        println!("✅ 原图已保存: {}", relative_path);

        // 获取保存后的文件大小
        let file_size = fs::metadata(&original_path)?.len();

        // Phase 2: 生成缩略图（同步，提高分辨率和质量）
        let thumbnail_img = img.thumbnail(800, 600);
        
        // 使用更高质量的 WebP 编码
        let mut webp_buffer = Vec::new();
        let webp_encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut webp_buffer);
        thumbnail_img.write_with_encoder(webp_encoder)
            .context("Failed to encode thumbnail")?;
        
        // 保存到文件
        fs::write(&thumbnail_path, &webp_buffer)
            .context("Failed to write thumbnail")?;
        println!("✅ 缩略图已生成: {}", relative_thumb_path);

        // 插入数据库记录
        let timestamp_micros = Utc::now().timestamp_micros();
        
        self.conn.execute(
            "INSERT INTO records (
                type, hash, created_at, content_text,
                image_path, thumbnail_path, image_format, image_size,
                image_hash, width, height, tag
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                ClipType::Image.to_string(),
                hash_hex, // hash字段用于通用去重
                timestamp_micros,
                format!("[图片] {}x{} {}", width, height, ext.to_uppercase()), // content_text用于预览
                relative_path,
                relative_thumb_path,
                ext,
                file_size as i64,
                hash_hex, // image_hash用于图片去重
                width as i64,
                height as i64,
                r#"["image"]"#, // tag标签
            ],
        )?;

        let id = self.conn.last_insert_rowid();
        println!("📸 图片记录已创建 ID: {}", id);
        
        // 返回 ID 和缩略图数据
        Ok((id, webp_buffer))
    }

    /// 根据 hash 查找已存在的图片
    fn find_image_by_hash(&self, hash: &str) -> Result<Option<i64>> {
        let result = self.conn
            .query_row(
                "SELECT id FROM records WHERE image_hash = ?1 AND type = 'image'",
                params![hash],
                |row| row.get(0),
            )
            .optional();
        
        match result {
            Ok(opt) => Ok(opt),
            Err(e) => {
                eprintln!("❌ 查询图片hash失败: {:?}", e);
                Err(anyhow::anyhow!("Failed to query image by hash: {}", e))
            }
        }
    }

}
