# UI 实现细节

**最后更新**: 2026-01-13  
**状态**: ✅ 基础功能完成

---

## 🎨 主题系统

### OneHalf Light (亮色模式)
```typescript
主背景: #fafafa          // 柔和浅灰白，不刺眼
卡片/输入框: #ffffff     // 纯白仅用于内容区
次级背景: #e5e5e6        // 标签/选中
底部状态栏: #f0f0f0      // 浅灰
边框: #e0e0e0 / #d0d0d0  // 柔和灰边框
主要文字: #383a42        // 深灰，易读
次要文字: #a0a1a7        // 中灰
高亮边框: #526fff        // 蓝色
置顶背景: #fef9c7        // 柔和黄色
```

### Dark Mode (暗黑模式)
```typescript
主背景: #1f1f1f          // 深色背景
次级背景: #2d2d2d        // 卡片/输入框
悬停背景: #3d3d3d        // 选中/标签
主要文字: #e0e0e0        // 亮灰
次要文字: #ccc           // 中亮灰
提示文字: #999 / #777    // 暗灰
边框: #444               // 深灰边框
置顶背景: #3d3d00        // 暗黄色
```

### 切换实现
- 使用 `useState<boolean>` 管理 `isDarkMode`
- 所有样式通过三元运算符动态切换
- 添加 `transition: 0.3s` 实现平滑过渡

---

## 🖼️ 图片显示

### 缩略图缓存策略
```typescript
interface ClipStore {
  thumbnailCache: Map<number, string>; // id -> base64 data URI
}

// 流程
1. clipboard://image-ready 事件携带 base64 编码的缩略图
2. 存入 thumbnailCache: `data:image/webp;base64,${thumbnail}`
3. ImagePreview 组件优先检查缓存
4. 缓存未命中时才通过 convertFileSrc 加载文件
```

### 异步加载三阶段
1. **pending**: 显示占位符 "处理中..." ⏳
2. **processing**: 后端生成 PNG + WebP
3. **ready**: emit base64，更新 UI

---

## 📊 列表功能

### 状态管理 (Zustand)
```typescript
interface ClipStore {
  allClips: ClipItem[];           // 主列表
  searchResults: ClipItem[];      // 搜索结果
  searchQuery: string;            // 搜索关键词
  filterType: string;             // 类型过滤 (Text/Html/Color/Image/Files)
  totalCount: number;             // 总记录数（不是 allClips.length）
  offset: number;                 // 分页偏移
  limit: number;                  // 每页20条
}
```

### 总计数实现
- **后端**: `get_total_count()` -> `SELECT COUNT(*) FROM records`
- **前端**: 
  - `fetchTotalCount()` 在初始化时调用
  - `clipboard://new-clip` 和 `image-ready` 事件触发时 +1
  - 显示在底部状态栏

### 分页
- 居中对齐：`justifyContent: "center"`
- 上一页/下一页按钮 + Offset 显示
- 仅在非搜索模式下显示

---

## 🎯 核心交互

### 搜索栏布局
```typescript
<div flex gap-8px>
  <input flex-1 />              // 搜索框
  <button>🗑️ 清空</button>      // 清空未置顶
  <label>🌙 <checkbox /></label> // 暗黑模式
  <label>🔒 <checkbox /></label> // 窗口保持
</div>
```

### 一键清空
```typescript
async handleClearUnpinned() {
  confirm("确定要清空所有未置顶的记录吗？")
  await invoke('clear_unpinned_clips') -> 返回删除条数
  重新 fetchAllClips() + fetchTotalCount()
}
```

### 窗口保持打开
```typescript
// 前端
const [keepWindowOpen, setKeepWindowOpen] = useState(false);
handleToggleKeepOpen(checked) -> invoke('set_keep_window_open', { keep })

// 后端
AppState { keep_window_open: Arc<Mutex<bool>> }
窗口失焦时检查 keep_window_open，为 true 则不隐藏
```

---

## 🎨 UI 组件细节

### Color 类型显示
- 圆形色块：32px × 32px
- `borderRadius: 50%`
- `backgroundColor: item.preview` (色值)
- 1px 边框防止白色不可见

### 列表项样式
```typescript
置顶项: 黄色背景高亮
loading: 半透明 + 禁用操作按钮
标签: 圆角小标签，背景色区分主题
操作按钮: 置顶/取消、删除（红色）
```

### 底部状态栏
```typescript
左侧: ⌨️ 快捷键提示 "Cmd+Shift+V 打开 | ESC 关闭"
右侧: 
  - 总记录数 / 搜索结果数
  - Sync 指示灯（黄色=处理中，绿色=空闲）
```

---

## 🔧 事件监听

### 三个核心 Listener
```typescript
// 1. 普通剪贴板事件
clipboard://new-clip -> fetchAllClips() + totalCount++

// 2. 图片处理开始
clipboard://image-pending -> 添加占位符到列表顶部

// 3. 图片处理完成
clipboard://image-ready -> 
  - 缓存 thumbnail base64
  - 移除占位符
  - totalCount++
  - fetchAllClips()
```

---

## 📦 依赖关系

```json
{
  "zustand": "5.0.10",           // 状态管理
  "@tauri-apps/api": "^2.0.0",  // IPC 通信
  "react": "^18.3.1",
  "typescript": "^5.7.2"
}
```

---

## ✅ 已实现功能清单

- [x] 暗黑/亮色主题切换（OneHalf Light + Dark）
- [x] 图片异步加载（pending → ready）
- [x] 缩略图 base64 缓存
- [x] 搜索 + 类型过滤
- [x] 分页（居中对齐）
- [x] 置顶/删除操作
- [x] 一键清空未置顶记录
- [x] 窗口保持打开开关
- [x] 总计数（实时更新）
- [x] Color 圆形预览（32px）
- [x] 底部状态栏（快捷键 + Sync 指示灯）

## 🔜 待优化

- [ ] 虚拟列表（virtua）- 性能优化
- [ ] 图片懒加载（Intersection Observer）
- [ ] 键盘导航（上下键选择，Enter 复制）
- [ ] 复制到剪贴板功能
- [ ] Auto-Paste 集成
