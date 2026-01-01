import React from "react";
import { Button } from "@/components/ui/button";

const ClipFooter: React.FC = () => {
  return (
    <footer className="h-10 bg-[#0f1a23]/90 backdrop-blur border-t border-white/5 flex items-center justify-between px-4 shrink-0 z-20">
      <Button variant="ghost" className="flex items-center gap-2 group text-gray-400 hover:text-white transition-colors">
        <span className="text-xs font-medium">Open Full View</span>
      </Button>
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-1.5 text-[10px] text-gray-500" title="Sync Status">
          <span className="w-1.5 h-1.5 rounded-full bg-emerald-500 shadow-[0_0_5px_rgba(16,185,129,0.5)]"></span>
          <span>Synced</span>
        </div>
        <Button variant="ghost" className="text-gray-400 hover:text-white hover:rotate-90 transition-all p-0 w-8 h-8 flex items-center justify-center">
          <span className="material-symbols-outlined text-[18px]">settings</span>
        </Button>
      </div>
    </footer>
  );
};

export default ClipFooter;
