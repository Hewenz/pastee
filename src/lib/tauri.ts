import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface ClipItemData {
    id: number;
    content_type: "Text" | "Image" | "Html" | "Files" | "Color";
    preview: string;
    created_at: number;
    is_pinned: boolean;
    tags: string[];
}

/**
 * 获取最近的剪贴板项
 */
export const getRecentClips = (limit: number = 20, offset: number = 0): Promise<ClipItemData[]> => {
    return invoke<ClipItemData[]>("get_recent_clips", { limit, offset });
};

/**
 * 搜索剪贴板项
 */
export const searchClips = (query: string): Promise<ClipItemData[]> => {
    return invoke<ClipItemData[]>("search_clips", { query });
};

/**
 * 获取完整内容
 */
export const getClipContent = (id: number): Promise<any> => {
    return invoke("get_clip_content", { id });
};

/**
 * 切换置顶状态
 */
export const togglePin = (id: number): Promise<boolean> => {
    return invoke<boolean>("toggle_pin", { id });
};

/**
 * 删除剪贴板项
 */
export const deleteClip = (id: number): Promise<void> => {
    return invoke("delete_clip", { id });
};

/**
 * 获取图片URL
 */
export const getImageUrl = async (id: number, thumbnail: boolean = false): Promise<string> => {
    const { convertFileSrc } = await import("@tauri-apps/api/core");
    const relativePath = await invoke<string>("get_image_url", { id, thumbnail });
    const { documentDir } = await import("@tauri-apps/api/path");
    const docDir = await documentDir();
    const fullPath = `${docDir}/pastee/images/${relativePath}`;
    return convertFileSrc(fullPath);
};

/**
 * 打开 macOS 辅助功能设置
 */
export const openAccessibilitySettings = (): Promise<void> => {
    return invoke("open_accessibility_settings");
};

/**
 * 监听剪贴板事件
 */
export const onClipboardNewClip = (callback: (data: any) => void): Promise<() => void> => {
    return listen("clipboard://new-clip", (event) => {
        callback(event.payload);
    });
};
