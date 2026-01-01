import React from "react";
import ClipApp from "./components/ClipApp";
import ClipHeader from "./components/ClipHeader";
import ClipList, { ClipGroup } from "./components/ClipList";
import ClipFooter from "./components/ClipFooter";
import "./App.css";

// 示例数据
const groups: ClipGroup[] = [
  {
    title: "Pinned",
    icon: "push_pin",
    items: [
      {
        type: "text",
        icon: "forum",
        title: "Q3 Design System Roadmap & Milestones",
        subtitle: "Slack • #product-design",
        time: "2h",
        pinned: true,
      },
      {
        type: "image",
        icon: "image",
        title: "Dashboard_Mockup_v4.png",
        subtitle: "Adobe Photoshop",
        imageUrl:
          "https://lh3.googleusercontent.com/aida-public/AB6AXuCqAIC3OG5CnzxEDN9wmuo9WQuj07wnZB9LhM3YhMnDoOiTFgJ_r-6nd9G4VMW8V3b97xQ9cB5o6N_qMJ1h1Y75V4XxSMnayz_co_ZU-rvIbC-VGHb_G0hZkKSPS-jTESER2LHwGrnQ0T6UtTW1pCYi8adcQSs1VeMtaVfqQHZKKWKnwkbxWzXoEcy_Wm5Iq8mcrpg4cHKLOsQ1SlWdy3ja-40sG6gdoOmWOxvj7CT3lhvos5ACzTv5iEaFvWRNv9JYmBWBKgKEnNSM",
        time: "1d",
      },
    ],
  },
  {
    title: "Today",
    icon: "history",
    items: [
      {
        type: "color",
        icon: "",
        title: "#0076D6",
        subtitle: "Figma • Color Value",
        colorValue: "#0076D6",
        time: "2m",
      },
      {
        type: "link",
        icon: "",
        title: "https://tailwindcss.com/docs/grid-template-columns",
        subtitle: "Google Chrome",
        time: "5m",
      },
      {
        type: "code",
        icon: "code",
        title: "npm install @heroicons/react",
        subtitle: "VS Code • Terminal",
        time: "15m",
      },
      {
        type: "file",
        icon: "folder",
        title: "C:\\Users\\Project\\Assets\\Logos",
        subtitle: "File Explorer",
        time: "1h",
      },
      {
        type: "text",
        icon: "description",
        title: "Please review the attached documents before...",
        subtitle: "System",
        time: "3h",
      },
    ],
  },
];

function App() {
  return (
    <ClipApp>
      <ClipHeader />
      <ClipList groups={groups} />
      <ClipFooter />
    </ClipApp>
  );
}

export default App;
