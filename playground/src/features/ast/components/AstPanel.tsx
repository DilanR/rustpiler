import { useState } from "react";

import type { AstNode } from "@/types";

import { AstGraph } from "./AstGraph";
import { AstTree } from "./AstView";

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
      <div className="ast-panel__toolbar">
        <div className="ast-panel__title">
          {visibleRoot.label}
        </div>

        <div className="ast-panel__toggle">
          {/* TODO: Include after Click code -> See node in Tree/graph done
          <button
            type="button"
            className="ast-panel__toggle-button"
            onClick={onReset}
            disabled={!selectedRoot}
          >
            Reset
          </button>
          */}

          <button
            type="button"
            className={
              mode === "tree"
                ? "ast-panel__toggle-button ast-panel__toggle-button--active"
                : "ast-panel__toggle-button"
            }
            onClick={() => setMode("tree")}
          >
            Tree
          </button>

          {/* TODO: Fix better Graph functionallity!
          <button
            type="button"
            className={
              mode === "graph"
                ? "ast-panel__toggle-button ast-panel__toggle-button--active"
                : "ast-panel__toggle-button"
            }
            onClick={() => setMode("graph")}
          >
            Graph
          </button>
          */}
        </div>
      </div>

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
