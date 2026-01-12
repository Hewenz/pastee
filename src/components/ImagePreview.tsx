import React, { useEffect, useState } from 'react';
import { getImageUrl } from '../lib/tauri';
import { useClipStore } from '../store/clipStore';

interface ImagePreviewProps {
  id: number;
  preview: string;
}

export const ImagePreview: React.FC<ImagePreviewProps> = ({ id, preview }) => {
  const thumbnailCache = useClipStore(state => state.thumbnailCache);
  const [thumbnailUrl, setThumbnailUrl] = useState<string>('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    // 优先使用缓存的缩略图
    const cachedThumbnail = thumbnailCache.get(id);
    if (cachedThumbnail) {
      setThumbnailUrl(cachedThumbnail);
      setLoading(false);
      return;
    }
    
    // 如果没有缓存，从文件加载
    getImageUrl(id, true)
      .then((url) => {
        setThumbnailUrl(url);
        setLoading(false);
      })
      .catch((err) => {
        console.error('Failed to load thumbnail:', err);
        setError(true);
        setLoading(false);
      });
  }, [id, thumbnailCache]);

  if (loading) {
    return (
      <div style={{ width: '160px', height: '120px', backgroundColor: '#f0f0f0', display: 'flex', alignItems: 'center', justifyContent: 'center', borderRadius: '4px' }}>
        <span style={{ fontSize: '11px', color: '#999' }}>加载中...</span>
      </div>
    );
  }

  if (error) {
    return (
      <div style={{ width: '160px', height: '120px', backgroundColor: '#f0f0f0', display: 'flex', alignItems: 'center', justifyContent: 'center', borderRadius: '4px' }}>
        <span style={{ fontSize: '20px' }}>🖼️</span>
      </div>
    );
  }

  return (
    <div style={{ width: '160px', height: '120px', overflow: 'hidden', borderRadius: '4px', border: '1px solid #ddd' }}>
      <img
        src={thumbnailUrl}
        alt={preview}
        style={{ width: '100%', height: '100%', objectFit: 'cover' }}
      />
    </div>
  );
};
