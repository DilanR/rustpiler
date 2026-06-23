import { useState } from "react";

import { useRustpiler } from "@/features/compiler";
import { findNodeAtPosition } from "@/features/ast/utils/findNodeAtPosition";

import type { AstNode, Range } from "@/types";

export function useWorkspace() {
  const compiler = useRustpiler();

  const [selectedRange, setSelectedRange] =
    useState<Range>();

  const [selectedAstNode, setSelectedAstNode] =
    useState<AstNode>();

  function handleSourceSelect(
    line: number,
    column: number
  ) {
    const ast = compiler.result?.ast;

    if (!ast) {
      return;
    }

    const node = findNodeAtPosition(
      ast,
      line,
      column
    );

    setSelectedAstNode(node);
    setSelectedRange(node?.span);
  }

  function handleAstSelect(
    node: AstNode
  ) {
    setSelectedAstNode(node);
    setSelectedRange(node.span);

    console.log(selectedAstNode)
  }

  function handleAstReset() {
    setSelectedAstNode(undefined);
    setSelectedRange(undefined);
  }

  function handleRun() {
    handleAstReset();
    compiler.run();
  }

  return {
    toolBarState: {
      loading: compiler.ready,
      onRun: handleRun,
      onSelect: compiler.setCode,
    },

    editorState: {
      code: compiler.code,
      setCode: compiler.setCode,
      onRun: handleRun,
      diagnostics: compiler.diagnostics,
      highlightedRange: selectedRange,
      onSourceSelect: handleSourceSelect,
    },

    sideBarState: {
      root: compiler.result?.ast,
      selectedRoot: selectedAstNode,
      onSelect: handleAstSelect,
      onReset: handleAstReset,
      diagnostics: compiler.diagnostics,
    },

    consoleState: {
      stdout: compiler.result?.stdout ?? "",
      result: String(
        compiler.result?.result ?? ""
      ),
    },
  };
}
