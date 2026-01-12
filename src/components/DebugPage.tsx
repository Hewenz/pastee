import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useClipStore } from "../store/clipStore";
import { ImagePreview } from "./ImagePreview";

const types = ["Text", "Html", "Color", "Image", "Files"];

// const handleOpenAccessibilitySettings = async () => {
//   try {
//     await invoke("open_accessibility_settings");
//   } catch (error) {
//     console.error("Failed to open accessibility settings:", error);
//     alert("无法打开辅助功能设置。请手动打开 系统设置 → 隐私与安全 → 辅助功能");
//   }
// };

export default function DebugPage() {
  const {
    // allClips,
    searchQuery,
    filterType,
    offset,
    limit,
    totalCount,
    displayList,
    setSearchQuery,
    setFilterType,
    setOffset,
    fetchAllClips,
    fetchTotalCount,
    handleDelete,
    handlePin,
    initListener,
  } = useClipStore();

  const list = displayList();
  const hasLoadingImages = list.some(item => item.loading);
  const [isDarkMode, setIsDarkMode] = useState(false);
  const [keepWindowOpen, setKeepWindowOpen] = useState(false);

  const handleClearUnpinned = async () => {
    if (!confirm('确定要清空所有未置顶的记录吗？此操作不可撤销。')) return;
    try {
      const deleted = await invoke<number>('clear_unpinned_clips');
      alert(`已清空 ${deleted} 条未置顶记录`);
      fetchAllClips();
      fetchTotalCount();
    } catch (error) {
      console.error('❌ 清空失败:', error);
      alert('清空失败，请重试');
    }
  };

  const handleToggleKeepOpen = async (checked: boolean) => {
    setKeepWindowOpen(checked);
    try {
      await invoke('set_keep_window_open', { keep: checked });
    } catch (error) {
      console.error('❌ 设置窗口保持失败:', error);
    }
  };

  useEffect(() => {
    fetchAllClips();
    fetchTotalCount();
  }, []);

  useEffect(() => {
    const cleanup = initListener();
    return () => {
      cleanup.then((unlisten) => unlisten());
    };
  }, []);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        invoke("toggle_window").catch(console.error);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  return (
    <div style={{ 
      fontFamily: "sans-serif", 
      fontSize: "14px", 
      display: "flex", 
      flexDirection: "column", 
      height: "100vh",
      backgroundColor: isDarkMode ? "#1f1f1f" : "#fafafa",
      color: isDarkMode ? "#e0e0e0" : "#383a42",
      transition: "background-color 0.3s, color 0.3s"
    }}>
      {/* 搜索框区域 */}
      <div style={{ display: "flex", alignItems: "center", gap: "8px", margin: "12px 12px 8px 12px" }}>
        <input 
          type="text" 
          value={searchQuery} 
          onChange={(e) => setSearchQuery(e.target.value)} 
          placeholder="搜索剪贴板内容..." 
          style={{ 
            flex: 1, 
            padding: "8px", 
            fontSize: "14px", 
            border: isDarkMode ? "1px solid #444" : "1px solid #d0d0d0", 
            borderRadius: "4px",
            backgroundColor: isDarkMode ? "#2d2d2d" : "#ffffff",
            color: isDarkMode ? "#e0e0e0" : "#383a42",
            transition: "background-color 0.3s, color 0.3s, border-color 0.3s"
          }} 
        />
        <button 
          onClick={handleClearUnpinned}
          style={{ 
            padding: "8px 12px", 
            fontSize: "13px", 
            backgroundColor: "#ff4444", 
            color: "#fff", 
            border: "none", 
            borderRadius: "4px", 
            cursor: "pointer",
            whiteSpace: "nowrap"
          }}
        >
          🗑️ 
        </button>
        <label style={{ display: "flex", alignItems: "center", gap: "6px", cursor: "pointer", whiteSpace: "nowrap" }}>
          <span style={{ fontSize: "13px" }}>🌙</span>
          <input 
            type="checkbox" 
            checked={isDarkMode} 
            onChange={(e) => setIsDarkMode(e.target.checked)}
            style={{ cursor: "pointer" }}
          />
        </label>
        <label style={{ display: "flex", alignItems: "center", gap: "6px", cursor: "pointer", whiteSpace: "nowrap" }}>
          <span style={{ fontSize: "13px" }} title="保持窗口打开">🔒</span>
          <input 
            type="checkbox" 
            checked={keepWindowOpen} 
            onChange={(e) => handleToggleKeepOpen(e.target.checked)}
            style={{ cursor: "pointer" }}
          />
        </label>
      </div>

      {/* 类型过滤 */}
      <div style={{ margin: "0 12px 12px 12px", display: "flex", gap: "6px", flexWrap: "wrap" }}>
        <button onClick={() => setFilterType("")} style={{ 
          padding: "4px 12px", 
          fontSize: "13px", 
          border: filterType === "" ? (isDarkMode ? "2px solid #e0e0e0" : "2px solid #526fff") : (isDarkMode ? "1px solid #444" : "1px solid #d0d0d0"), 
          borderRadius: "4px", 
          backgroundColor: filterType === "" ? (isDarkMode ? "#3d3d3d" : "#e5e5e6") : (isDarkMode ? "#2d2d2d" : "#ffffff"), 
          color: isDarkMode ? "#e0e0e0" : "#383a42",
          cursor: "pointer",
          transition: "all 0.3s"
        }}>全部</button>
        {types.map((type) => (
          <button key={type} onClick={() => setFilterType(type)} style={{ 
            padding: "4px 12px", 
            fontSize: "13px", 
            border: filterType === type ? (isDarkMode ? "2px solid #e0e0e0" : "2px solid #526fff") : (isDarkMode ? "1px solid #444" : "1px solid #d0d0d0"), 
            borderRadius: "4px", 
            backgroundColor: filterType === type ? (isDarkMode ? "#3d3d3d" : "#e5e5e6") : (isDarkMode ? "#2d2d2d" : "#ffffff"), 
            color: isDarkMode ? "#e0e0e0" : "#383a42",
            cursor: "pointer",
            transition: "all 0.3s"
          }}>{type}</button>
        ))}
      </div>

      {/* 分页 */}
      {!searchQuery.trim() && (
        <div style={{ margin: "0 12px 8px 12px", fontSize: "13px", display: "flex", justifyContent: "center", alignItems: "center", gap: "8px" }}>
          <button onClick={() => setOffset(Math.max(0, offset - limit))} disabled={offset === 0} style={{ 
            padding: "4px 10px", 
            fontSize: "13px",
            backgroundColor: isDarkMode ? "#2d2d2d" : "#ffffff",
            color: isDarkMode ? "#e0e0e0" : "#383a42",
            border: isDarkMode ? "1px solid #444" : "1px solid #d0d0d0",
            borderRadius: "4px",
            cursor: offset === 0 ? "not-allowed" : "pointer",
            opacity: offset === 0 ? 0.5 : 1
          }}>上一页</button>
          <span style={{ color: isDarkMode ? "#999" : "#a0a1a7" }}>Offset: {offset}</span>
          <button onClick={() => setOffset(offset + limit)} style={{ 
            padding: "4px 10px", 
            fontSize: "13px",
            backgroundColor: isDarkMode ? "#2d2d2d" : "#ffffff",
            color: isDarkMode ? "#e0e0e0" : "#383a42",
            border: isDarkMode ? "1px solid #444" : "1px solid #d0d0d0",
            borderRadius: "4px",
            cursor: "pointer"
          }}>下一页</button>
        </div>
      )}

      {/* 列表 */}
      <div style={{ flex: 1, overflowY: "auto", margin: "0 12px 0 12px" }}>
      {list.length === 0 ? (
        <p style={{ color: isDarkMode ? "#666" : "#a0a1a7", fontSize: "13px" }}>{searchQuery.trim() ? "无搜索结果" : "暂无数据"}</p>
      ) : (
        <ul style={{ listStyle: "none", padding: 0, margin: 0 }}>
          {list.map((item) => (
            <li key={item.id} style={{ 
              border: isDarkMode ? "1px solid #444" : "1px solid #e0e0e0", 
              borderRadius: "3px", 
              padding: "8px", 
              marginBottom: "6px", 
              backgroundColor: item.is_pinned ? (isDarkMode ? "#3d3d00" : "#fef9c7") : (isDarkMode ? "#2d2d2d" : "#ffffff"), 
              fontSize: "13px",
              transition: "background-color 0.3s, border-color 0.3s"
            }}>
              <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "4px", fontSize: "12px" }}>
                <div>
                  <strong style={{ fontSize: "13px", color: isDarkMode ? "#e0e0e0" : "#383a42" }}>ID:{item.id}</strong>
                  <span style={{ marginLeft: "8px", color: isDarkMode ? "#999" : "#a0a1a7" }}>{item.content_type}</span>
                  {item.is_pinned && <span style={{ marginLeft: "6px" }}>📌</span>}
                </div>
                <div style={{ fontSize: "11px", color: isDarkMode ? "#777" : "#a0a1a7" }}>{new Date(item.created_at / 1000).toLocaleString()}</div>
              </div>
              <div style={{ marginBottom: "4px", color: isDarkMode ? "#ccc" : "#383a42", fontSize: "13px", wordBreak: "break-word", display: "flex", alignItems: "center", gap: "8px" }}>
                {item.content_type === "Color" && (
                  <div style={{ 
                    width: "32px", 
                    height: "32px", 
                    borderRadius: "50%", 
                    backgroundColor: item.preview,
                    border: "1px solid #ddd",
                    flexShrink: 0
                  }} />
                )}
                {item.loading ? (
                  <div style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '20px', color: isDarkMode ? '#777' : '#a0a1a7' }}>
                    <span style={{ fontSize: '16px' }}>⏳</span>
                    <span>处理中...</span>
                  </div>
                ) : item.content_type === "Image" ? (
                  <ImagePreview id={item.id} preview={item.preview} />
                ) : (
                  <span>{item.preview}</span>
                )}
              </div>
              <div style={{ display: "flex", gap: "4px", alignItems: "center", justifyContent: "space-between" }}>
                <div style={{ display: "flex", gap: "4px", flexWrap: "wrap" }}>
                  {item.tags.map((tag) => (
                    <span key={tag} style={{ 
                      display: "inline-block", 
                      padding: "2px 8px", 
                      backgroundColor: isDarkMode ? "#3d3d3d" : "#e5e5e6", 
                      color: isDarkMode ? "#ccc" : "#383a42",
                      borderRadius: "3px", 
                      fontSize: "12px", 
                      fontWeight: "500",
                      transition: "background-color 0.3s, color 0.3s"
                    }}>{tag}</span>
                  ))}
                </div>
                <div style={{ display: "flex", gap: "4px", marginLeft: "auto" }}>
                  <button onClick={() => handlePin(item.id)} disabled={item.loading} style={{ 
                    padding: "2px 6px", 
                    fontSize: "11px", 
                    border: isDarkMode ? "1px solid #444" : "1px solid #d0d0d0", 
                    borderRadius: "3px", 
                    backgroundColor: item.loading ? (isDarkMode ? "#3d3d3d" : "#e5e5e6") : (isDarkMode ? "#2d2d2d" : "#ffffff"), 
                    color: isDarkMode ? "#e0e0e0" : "#383a42",
                    cursor: item.loading ? "not-allowed" : "pointer", 
                    opacity: item.loading ? 0.5 : 1,
                    transition: "all 0.3s"
                  }}>{item.is_pinned ? "取消" : "置顶"}</button>
                  <button onClick={() => handleDelete(item.id)} disabled={item.loading} style={{ padding: "2px 6px", fontSize: "11px", backgroundColor: item.loading ? "#ccc" : "#ff4444", color: "#fff", border: "none", borderRadius: "3px", cursor: item.loading ? "not-allowed" : "pointer", opacity: item.loading ? 0.5 : 1 }}>删除</button>
                </div>
              </div>
            </li>
          ))}
        </ul>
      )}
      </div>

      {/* 底部状态栏 */}
      <div style={{ 
        borderTop: isDarkMode ? "1px solid #444" : "1px solid #e0e0e0", 
        padding: "8px 12px", 
        display: "flex", 
        justifyContent: "space-between", 
        alignItems: "center",
        fontSize: "12px",
        color: isDarkMode ? "#999" : "#a0a1a7",
        backgroundColor: isDarkMode ? "#2d2d2d" : "#f0f0f0",
        marginTop: "8px",
        transition: "background-color 0.3s, color 0.3s, border-color 0.3s"
      }}>
        <div>
          <span style={{ fontWeight: "500" }}>⌨️ 快捷键: </span>
          <span>Cmd+Shift+V 打开 | ESC 关闭</span>
        </div>
        <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
          <span>
            {searchQuery.trim() 
              ? `搜索结果: ${list.length} 条` 
              : `总记录: ${totalCount} 条`}
          </span>
          <div style={{ display: "flex", alignItems: "center", gap: "4px" }}>
            <span>Sync</span>
            <div style={{ 
              width: "10px", 
              height: "10px", 
              borderRadius: "50%", 
              backgroundColor: hasLoadingImages ? "#faad14" : "#52c41a",
              transition: "background-color 0.3s"
            }} />
          </div>
        </div>
      </div>
    </div>
  );
}
