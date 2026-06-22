import { useState } from "react";

import type { AstNode } from "@/types";

import { AstGraph } from "./AstGraph";
import { AstTree } from "./AstView";
import "../index.css"

type AstViewMode = "tree" | "graph";

type Props = {
  root?: AstNode;
  selectedRoot?: AstNode;
  onSelect?: (node: AstNode) => void;
  onReset?: () => void;
};

export function AstPanel({
  root,
  selectedRoot,
  onSelect,
  /*onReset,*/
}: Props) {
  const [mode, setMode] =
    useState<AstViewMode>("tree");

  if (!root) {
    return null;
  }

  const visibleRoot =
    selectedRoot ?? root;

  /*const viewKey = selectedRoot
    ? `${selectedRoot.kind}:${selectedRoot.label}:${selectedRoot.span?.start_line ?? 0}:${selectedRoot.span?.start_column ?? 0}`
    : "program";
    */

  return (
    <>
      <div className="ast-panel__content">
        {mode === "tree" ? (
          <AstTree
            node={root}
            onSelect={onSelect}
          />
        ) : (
          <AstGraph
            node={root}
            onSelect={onSelect}
          />
        )}
      </div>
    </>
  );
}
