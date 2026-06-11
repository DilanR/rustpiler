import type { AstNode } from "@/types";

export function findNodeAtPosition(
  node: AstNode,
  line: number,
  column: number
): AstNode | undefined {
  const childMatch = node.children
    .map((child) =>
      findNodeAtPosition(
        child,
        line,
        column
      )
    )
    .find((child) => child);

  if (childMatch) {
    return childMatch;
  }

  if (
    node.span &&
    containsPosition(
      node.span,
      line,
      column
    )
  ) {
    return node;
  }

  return undefined;
}

function containsPosition(
  range: AstNode["span"],
  line: number,
  column: number
): boolean {
  if (!range) {
    return false;
  }

  const startsBefore =
    range.start_line < line ||
    (
      range.start_line === line &&
      range.start_column <= column
    );

  const endsAfter =
    range.end_line > line ||
    (
      range.end_line === line &&
      range.end_column >= column
    );

  return startsBefore && endsAfter;
}
