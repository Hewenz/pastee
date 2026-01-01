import React from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

const filterChips = [
	{ icon: "check", label: "All Types", active: true },
	{ icon: "text_fields", label: "Text" },
	{ icon: "image", label: "Images" },
	{ icon: "link", label: "Links" },
	{ icon: "folder", label: "Files" },
];

const ClipHeader: React.FC = () => {
	return (
		<header className="flex flex-col gap-3 px-4 pt-5 pb-3 bg-linear-to-b from-[#0f1a23]/90 to-transparent shrink-0">
			{/* 搜索输入框 */}
			<div className="relative group">
				<Input
					className="block w-full py-2.5 pl-4 pr-4 text-sm bg-black/20 border border-white/10 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-1 focus:ring-primary focus:border-primary focus:bg-black/40 transition-all shadow-inner"
					placeholder="Search clips..."
					type="text"
				/>
				<div className="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
					<span className="text-[10px] text-gray-500 bg-white/5 border border-white/5 px-1.5 py-0.5 rounded">
						/
					</span>
				</div>
			</div>
			{/* 过滤 Chips */}
			<div className="flex gap-2 overflow-x-auto no-scrollbar pb-1 mask-linear-fade">
				{filterChips.map((chip) => (
					<Button
						key={chip.label}
						variant={chip.active ? "default" : "outline"}
						className={`flex items-center gap-1.5 text-xs font-medium rounded-full whitespace-nowrap px-3 py-1.5 ${
							chip.active
								? "bg-primary/20 hover:bg-primary/30 border-primary/30"
								: "bg-white/5 hover:bg-white/10 hover:text-white border-white/5 text-gray-400"
						}`}
					>
						{chip.label}
					</Button>
				))}
			</div>
		</header>
	);
};

export default ClipHeader;
