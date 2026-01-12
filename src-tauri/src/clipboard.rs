use arboard::Clipboard;
use clipboard_master::{CallbackResult, ClipboardHandler};
use crossbeam_channel::Sender;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// 定义传递给主线程的数据类型
#[derive(Debug)]
pub enum ClipEvent {
    Text(String),
    Image { width: usize, height: usize, rgba_data: Vec<u8> }, // RGBA 原始数据
    Html(String),
    FileList(Vec<std::path::PathBuf>),
    Error(String),
}

// 监听器结构体
pub struct SystemHook {
    // 通信管道发送端
    pub sender: Sender<ClipEvent>,
    // 用于防抖 (Debounce)：记录上一次内容的哈希和时间
    pub last_hash: Arc<Mutex<String>>,
    pub last_update: Arc<Mutex<Instant>>,
}

impl SystemHook {
    pub fn new(sender: Sender<ClipEvent>) -> Self {
        Self {
            sender,
            last_hash: Arc::new(Mutex::new(String::new())),
            last_update: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub fn update_latest(&self, data: &[u8]) -> bool {
        let hash = compute_hash(data);
        let now = Instant::now();
        
        let mut last_hash_guard = self.last_hash.lock().unwrap();
        let mut last_time_guard = self.last_update.lock().unwrap();

        if *last_hash_guard == hash && now.duration_since(*last_time_guard) < Duration::from_millis(500) {
            return false;
        }


        *last_hash_guard = hash;
        *last_time_guard = now;
        return true;
    }

}

impl ClipboardHandler for SystemHook {
    // 核心：当系统发生“复制”事件时，操作系统自动回调此函数
    fn on_clipboard_change(&mut self) -> CallbackResult {
        println!(">> ⚡ 底层事件触发 (Hook Triggered)");

        // 1. 初始化读取器 (每次读取都建议新建实例以获取最新状态)
        let mut ctx = match Clipboard::new() {
            Ok(ctx) => ctx,
            Err(e) => {
                let _ = self.sender.send(ClipEvent::Error(e.to_string()));
                return CallbackResult::Next;
            }
        };

        if let Ok(file_list) = ctx.get().file_list() {
            let paths_str = file_list.iter()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>().join("\n");
            if !self.update_latest(paths_str.as_bytes()) {
                return CallbackResult::Next;
            }
            let _ = self.sender.send(ClipEvent::FileList(file_list));
        }
        else if let Ok(img) = ctx.get_image() {
            let data = img.bytes.to_vec();
            if !self.update_latest(&data) {
                return CallbackResult::Next;
            }
            let _ = self.sender.send(ClipEvent::Image {
                width: img.width,
                height: img.height,
                rgba_data: data,
            });
        }
        else if let Ok(html) = ctx.get().html() {
            if !self.update_latest(html.as_bytes()) {
                return CallbackResult::Next;
            }
            let _ = self.sender.send(ClipEvent::Html(html));
        }
        else if let Ok(text) = ctx.get_text() {        // Text 作为兜底
            if !self.update_latest(text.as_bytes()) {
                return CallbackResult::Next;
            }
            let _ = self.sender.send(ClipEvent::Text(text));
        } else {
            eprintln!("未知类型");
        }

        // 继续监听下一条消息
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: std::io::Error) -> CallbackResult {
        eprintln!("Hook Error: {}", error);
        CallbackResult::Next
    }
}

// 辅助函数：计算哈希 (使用 Blake3)
fn compute_hash(data: &[u8]) -> String {
    let hash = blake3::hash(data);
    hex::encode(hash.as_bytes())
}