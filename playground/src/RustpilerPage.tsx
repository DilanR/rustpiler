import { useRustpiler } from "@/features/compiler";
import { useState } from "react";
import type { AstNode, Range } from "@/types";
import { EditorWorkspace } from "./features/editor/components/EditorWorkspace";
import { findNodeAtPosition } from "./features/ast/utils/findNodeAtPosition";

export function RustpilerPage() {
  const rustpiler = useRustpiler();

  const [selectedRange, setSelectedRange] =
    useState<Range>();

  const [selectedAstNode, setSelectedAstNode] =
    useState<AstNode>();

  function handleSourceSelect(
    line: number,
    column: number
  ) {
    const ast =
      rustpiler.result?.ast;

    if (!ast) {
      return;
    }

    const node =
      findNodeAtPosition(
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
  }

  function handleAstReset() {
    setSelectedAstNode(undefined);
    setSelectedRange(undefined);
  }

  function handleRun() {
    setSelectedAstNode(undefined);
    setSelectedRange(undefined);
    rustpiler.run();
  }

  return (
    <EditorWorkspace
      code={rustpiler.code}
      setCode={rustpiler.setCode}
      onRun={handleRun}
      loading={rustpiler.ready}
      diagnostics={rustpiler.diagnostics}
      highlightedRange={selectedRange}
      ast={rustpiler.result?.ast}
      selectedAstNode={selectedAstNode}
      onSourceSelect={handleSourceSelect}
      onAstSelect={handleAstSelect}
      onAstReset={handleAstReset}
      stdout={rustpiler.result?.stdout}
      result={
        String(
          rustpiler.result?.result ??
          ""
        )
      }
    />
  );
}
