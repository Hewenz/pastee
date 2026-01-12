import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface ClipItem {
  id: number;
  content_type: string;
  preview: string;
  created_at: number;
  is_pinned: boolean;
  tags: string[];
  loading?: boolean; // 图片处理中标识
  temp_id?: number;  // 临时 ID，用于匹配处理中的图片
}

interface ClipStore {
  // State
  allClips: ClipItem[];
  searchResults: ClipItem[];
  searchQuery: string;
  filterType: string;
  limit: number;
  offset: number;
  thumbnailCache: Map<number, string>; // 缓存 id -> base64 缩略图
  totalCount: number; // 总记录数
  
  // Computed
  displayList: () => ClipItem[];
  
  // Actions
  setSearchQuery: (query: string) => void;
  setFilterType: (type: string) => void;
  setOffset: (offset: number) => void;
  fetchAllClips: () => Promise<void>;
  fetchTotalCount: () => Promise<void>;
  handleSearch: (query: string) => Promise<void>;
  handleDelete: (id: number) => Promise<void>;
  handlePin: (id: number) => Promise<void>;
  initListener: () => Promise<() => void>;
}

export const useClipStore = create<ClipStore>((set, get) => ({
  // Initial state
  allClips: [],
  searchResults: [],
  searchQuery: '',
  filterType: '',
  limit: 20,
  offset: 0,
  thumbnailCache: new Map(),
  totalCount: 0,
  
  // Computed
  displayList: () => {
    const { searchQuery, searchResults, allClips, filterType } = get();
    let list = searchQuery.trim() ? searchResults : allClips;
    if (filterType) {
      list = list.filter(item => item.content_type === filterType);
    }
    return list;
  },
  
  // Actions
  setSearchQuery: (query) => {
    set({ searchQuery: query });
    get().handleSearch(query);
  },
  
  setFilterType: (type) => set({ filterType: type }),
  
  setOffset: (offset) => {
    set({ offset });
    get().fetchAllClips();
  },
  
  fetchAllClips: async () => {
    try {
      const { limit, offset } = get();
      console.log('🔍 fetchAllClips 调用: limit=', limit, 'offset=', offset);
      const result = await invoke<ClipItem[]>('get_recent_clips', { limit, offset });
      console.log('✅ fetchAllClips 返回:', result.length, '条记录', result);
      set({ allClips: result });
    } catch (error) {
      console.error('❌ 获取历史失败:', error);
    }
  },

  fetchTotalCount: async () => {
    try {
      const count = await invoke<number>('get_total_count');
      console.log('📊 总记录数:', count);
      set({ totalCount: count });
    } catch (error) {
      console.error('❌ 获取总数失败:', error);
    }
  },
  
  handleSearch: async (query: string) => {
    if (!query.trim()) {
      set({ searchResults: [] });
      return;
    }
    try {
      const result = await invoke<ClipItem[]>('search_clips', { query });
      set({ searchResults: result });
    } catch (error) {
      console.error('❌ 搜索失败:', error);
    }
  },
  
  handleDelete: async (id: number) => {
    if (!confirm(`确定删除 ID ${id}?`)) return;
    try {
      await invoke('delete_clip', { id });
      await get().fetchAllClips();
      const { searchQuery } = get();
      if (searchQuery) {
        await get().handleSearch(searchQuery);
      }
    } catch (error) {
      console.error('❌ 删除失败:', error);
    }
  },
  
  handlePin: async (id: number) => {
    try {
      await invoke('toggle_pin', { id });
      await get().fetchAllClips();
      const { searchQuery } = get();
      if (searchQuery) {
        await get().handleSearch(searchQuery);
      }
    } catch (error) {
      console.error('❌ 置顶操作失败:', error);
    }
  },
  
  initListener: async () => {
    // 监听普通剪贴板事件（文本、HTML、文件等）
    const unlistenNormal = await listen<any>('clipboard://new-clip', () => {
      get().fetchAllClips();
      const { searchQuery, totalCount } = get();
      set({ totalCount: totalCount + 1 }); // 增加总数
      if (searchQuery) {
        get().handleSearch(searchQuery);
      }
    });
    
    // 监听图片处理开始事件
    const unlistenImagePending = await listen<any>('clipboard://image-pending', (event) => {
      const { temp_id } = event.payload;
      const { allClips } = get();
      
      // 添加一个占位项到列表顶部
      const placeholderItem: ClipItem = {
        id: 0,
        temp_id: temp_id,
        content_type: 'Image',
        preview: '处理中...',
        created_at: Date.now() * 1000,
        is_pinned: false,
        tags: ['image'],
        loading: true
      };
      
      set({ allClips: [placeholderItem, ...allClips] });
    });
    
    // 监听图片处理完成事件
    const unlistenImageReady = await listen<any>('clipboard://image-ready', (event) => {
      const { temp_id, id, thumbnail } = event.payload;
      const { allClips, thumbnailCache, totalCount } = get();
      
      // 缓存缩略图
      if (thumbnail) {
        thumbnailCache.set(id, `data:image/webp;base64,${thumbnail}`);
      }
      
      // 移除占位项，刷新列表
      const filteredClips = allClips.filter(item => item.temp_id !== temp_id);
      set({ 
        allClips: filteredClips,
        thumbnailCache: new Map(thumbnailCache),
        totalCount: totalCount + 1 // 增加总数
      });
      
      // 重新获取完整列表
      get().fetchAllClips();
    });
    
    // 监听图片处理错误事件
    const unlistenImageError = await listen<any>('clipboard://image-error', (event) => {
      const { temp_id, error } = event.payload;
      const { allClips } = get();
      
      console.error('图片处理失败:', error);
      
      // 移除占位项
      const filteredClips = allClips.filter(item => item.temp_id !== temp_id);
      set({ allClips: filteredClips });
    });
    
    // 返回清理函数
    return () => {
      unlistenNormal();
      unlistenImagePending();
      unlistenImageReady();
      unlistenImageError();
    };
  },
}));
