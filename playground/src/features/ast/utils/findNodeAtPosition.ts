import type { AstNode } from "@/types";

export function findNodeAtPosition(
  node: AstNode,
  line: number,
  column: number
): AstNode | undefined
