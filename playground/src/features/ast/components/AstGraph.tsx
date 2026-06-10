import { useRef } from "react";

import type { AstNode }
  from "@/types";

import { useExpandedAst }
  from "../hooks/useExpandedAst";

import { useVisibleAst }
  from "../hooks/useVisibleAst";

import { useD3AstGraph }
  from "../hooks/useD3AstGraph";

type Props = {
  node?: AstNode;
  onSelect?: (
    node: AstNode
  ) => void;
};

export function AstGraph({
  node,
  onSelect,
}: Props) {
  const svgRef =
    useRef<SVGSVGElement>(
      null
    );

  const {
    expanded,
    toggleNode,
  } = useExpandedAst();

  const visibleTree =
    useVisibleAst(
      node,
      expanded
    );

  useD3AstGraph(
    svgRef,
    visibleTree,
    (
      node,
      path
    ) => {
      toggleNode(path);
      onSelect?.(node);
    }
  );

  if (!node) {
    return null;
  }

  return (
    <svg
      ref={svgRef}
      className="ast-graph"
    />
  );
}
