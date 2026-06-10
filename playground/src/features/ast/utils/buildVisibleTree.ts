import type { AstNode } from "@/types";

export function buildVisibleTree(
  node: AstNode,
  expanded: Set<string>,
  path = "root"
): AstNode {
  return {
    ...node,
    children: expanded.has(path)
      ? node.children.map(
        (child, i) =>
          buildVisibleTree(
            child,
            expanded,
            `${path}.${i}`
          )
      )
      : [],
  };
}
