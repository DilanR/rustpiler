import type { EditorTab } from "@/types";
import "../index.css";

type Props = {
  activeTab: EditorTab;
  onChange: (tab: EditorTab) => void;
};

const tabs: EditorTab[] = [
  "source",
  "output",
  "ast",
  "typecheck",
];

export function EditorTabs({
  activeTab,
  onChange,
}: Props) {
  return (
    <div className="editor-tabs">
      {tabs.map((tab) => (
        <button
          key={tab}
          onClick={() =>
            onChange(tab)
          }
          className={
            activeTab === tab
              ? "editor-tab editor-tab--active"
              : "editor-tab"
          }
        >
          {tab}
        </button>
      ))}
    </div>
  );
}
