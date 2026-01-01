import React from "react";
import { Button } from "@/components/ui/button";
import ReactSvg from "../assets/react.svg";


export interface ClipItemData {
  type: "text" | "image" | "link" | "file" | "color" | "code";
  icon: string;
  title: string;
  subtitle?: string;
  imageUrl?: string;
  colorValue?: string;
  time: string;
  pinned?: boolean;
}

const ClipItem: React.FC<{ item: ClipItemData }> = ({ item }) => {
  return (
    <div className="group relative flex items-center gap-3 p-2 rounded-lg hover:bg-white/10 transition-colors cursor-default border border-transparent hover:border-white/5">
      {/* 图标或缩略图 */}
      <div className={`shrink-0 w-9 h-9 rounded-lg flex items-center justify-center ${item.type === "image" ? "overflow-hidden relative bg-[#001e36] border border-white/10" : item.type === "color" ? "bg-black border border-white/10" : item.type === "file" ? "bg-[#e8c04d] text-black border border-white/5" : item.type === "code" ? "bg-[#2C2C32] border border-white/5" : item.type === "link" ? "bg-[#1e293b] border border-white/5" : "bg-[#1e2e3e] border border-white/5"}`}>
        <img alt="Preview" className="w-full h-full opacity-80" src={ReactSvg} />
        {item.type === "image" && (
          <div className="absolute inset-0 flex items-center justify-center bg-black/30">
            <span className="material-symbols-outlined text-white text-[16px] drop-shadow-md">image</span>
          </div>
        )}
      </div>
      {/* 内容 */}
      <div className="flex-1 min-w-0 pr-12">
        <p className={`text-sm ${item.type === "code" ? "font-mono text-gray-300" : "text-white font-normal"} truncate`}>{item.title}</p>
        {item.subtitle && (
          <div className="flex items-center gap-2 mt-0.5">
            <span className="text-[11px] text-gray-400 truncate">{item.subtitle}</span>
          </div>
        )}
      </div>
      {/* 操作按钮 */}
      <div className="absolute right-3 top-1/2 -translate-y-1/2 flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity bg-background-dark/80 backdrop-blur rounded-lg pl-2 border border-white/10">
        <Button variant="ghost" size="icon-sm" className="text-primary" title="Pin"><span className="material-symbols-outlined text-[16px] fill-1">push_pin</span></Button>
        <Button variant="ghost" size="icon-sm" className="text-white" title="Copy"><span className="material-symbols-outlined text-[16px]">content_copy</span></Button>
        <Button variant="ghost" size="icon-sm" className="text-gray-400 hover:text-red-400" title="Delete"><span className="material-symbols-outlined text-[16px]">delete</span></Button>
      </div>
      <span className="text-[11px] text-gray-500 absolute right-3 top-3 group-hover:opacity-0 transition-opacity">{item.time}</span>
    </div>
  );
};

export default ClipItem;
