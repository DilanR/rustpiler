import { useState } from "react";
import type { EditorTab } from "@/types";

export function useEditorTabs() {
  const [activeTab, setActiveTab] =
    useState<EditorTab>("source");

  return {
    activeTab,
    setActiveTab,
  };
}
