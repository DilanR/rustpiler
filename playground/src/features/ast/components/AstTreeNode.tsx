import { useState } from "react";
import type { AstNode } from "@/types";

type Props = {
  node: AstNode;
  onSelect?: (node: AstNode) => void;
};

function labelClass(kind: string): string {
  switch (kind) {
    case "Prog":
      return "ast-tree__label--prog";

    case "Function":
      return "ast-tree__label--function";

    case "Ident":
      return "ast-tree__label--ident";

    case "Literal":
      return "ast-tree__label--literal";

    case "Type":
      return "ast-tree__label--type";

    case "BinOp":
      return "ast-tree__label--binop";

    case "Let":
    case "Assign":
    case "While":
      return "ast-tree__label--stmt";

    default:
      return "";
  }
}

export function AstTreeNode({
  node,
  onSelect,
}: Props) {
  const [expanded, setExpanded] =
    useState(true);

  const hasChildren =
    node.children.length > 0;

  return (
    <ul className="ast-tree">
      <li>
        <div
          className="ast-tree__node"
          onClick={(e) => {
            e.stopPropagation();

            if (hasChildren) {
              setExpanded(!expanded);
            }

            onSelect?.(node);
          }}
        >
          <span className="ast-tree__arrow">
            {hasChildren
              ? expanded
                ? "▼"
                : "▶"
              : "•"}
          </span>

          <span
            className={`ast-tree__label ${labelClass(
              node.kind
            )}`}
          >
            {node.label}
          </span>

          {node.span && (
            <span className="ast-tree__span">
              [{node.span.start_line}:
              {node.span.start_column}-
              {node.span.end_line}:
              {node.span.end_column}]
            </span>
          )}
        </div>

        {expanded &&
          hasChildren && (
            <div className="ast-tree__children">
              {node.children.map(
                (child, index) => (
                  <AstTreeNode
                    key={index}
                    node={child}
                    onSelect={onSelect}
                  />
                )
              )}
            </div>
          )}
      </li>
    </ul>
  );
}
