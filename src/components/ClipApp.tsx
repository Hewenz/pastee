import React from "react";

const ClipApp: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return (
    <div className="h-screen w-screen overflow-hidden flex flex-col">
      {/* 主窗口内容 */}
      <main className="bg-[#101b24] text-white flex-1 flex flex-col">
        {children}
      </main>
    </div>
  );
};

export default ClipApp;
