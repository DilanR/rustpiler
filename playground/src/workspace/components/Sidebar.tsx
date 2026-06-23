import { useState } from "react";

import { AstTree } from "@/features/ast";

import { SidebarToolbar } from "./SidebarToolbar";

import type { AstNode, Diagnostic } from "@/types";
import { DiagnosticsView } from "@/features/diagnostics/components/DiagnosticsView";

export type SidebarView =
  | "ast"
  | "diagnostics"
  | "mips asm";

type Props = {
  root?: AstNode;
  selectedRoot?: AstNode;
  onSelect: (node: AstNode) => void;
  onReset: () => void;
  diagnostics: Diagnostic[];
};

export function Sidebar({
  root,
  onSelect,
  diagnostics,
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
        diagnosticsCount={diagnostics.length}
        collapsed={collapsed}
        onToggle={() =>
          setCollapsed(v => !v)
        }
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

          {view === "diagnostics" && (
            <DiagnosticsView
              diagnostics={diagnostics}
            />
          )}
        </div>
      )}


    </div >
  );
}
