import { useState } from "react";

import { AstTree } from "@/features/ast";

import { SidebarToolbar } from "./SidebarToolbar";

import type { AstNode } from "@/types";

export type SidebarView =
  | "ast"
  | "diagnostics"
  | "mips asm";

type Props = {
  root?: AstNode;
  selectedRoot?: AstNode;
  onSelect: (node: AstNode) => void;
  onReset: () => void;
};

export function Sidebar({
  root,
  onSelect,
}: Props) {
  const [view, setView] =
    useState<SidebarView>("ast");

  const [collapsed, setCollapsed] =
    useState(false);

  if (!root) {
    return null;
  }

  return (
    <div
      className={
        collapsed
          ? "sidebar sidebar--collapsed"
          : "sidebar"
      }
    >
      <SidebarToolbar
        collapsed={collapsed}
        onToggle={() =>
          setCollapsed(v => !v)
        }
        view={view}
        onChange={setView}
      />

      {!collapsed && (
        <div className="sidebar__content">
          {view === "ast" && (
            <AstTree
              node={root}
              onSelect={onSelect}
            />
          )}
        </div>
      )}


    </div >
  );
}
