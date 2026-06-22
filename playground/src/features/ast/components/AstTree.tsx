import { useState } from "react";
import type { AstNode } from "@/types";

type Props = {
  node?: AstNode;
  onSelect?: (node: AstNode) => void;
};

function nodeColor(label: string): string {
  if (label.startsWith("Ident")) {
    return "#61afef";
  }

  if (label.startsWith("Literal")) {
    return "#98c379";
  }

  if (label.startsWith("BinOp")) {
    return "#e5c07b";
  }

  if (label.startsWith("Type")) {
    return "#c678dd";
  }

  if (
    label.startsWith("Let") ||
    label.startsWith("Assign") ||
    label.startsWith("While")
  ) {
    return "#e06c75";
  }

  return "inherit";
}

export function AstTree({
  node,
  onSelect,
}: Props) {
  const [expanded, setExpanded] =
    useState(true);

  if (!node) {
    return (
      <div
        style={{
          fontFamily: "monospace",
        }}
      >
        no ast
      </div>
    );
  }

  const hasChildren =
    node.children.length > 0;

  return (
    <ul
      style={{
        listStyle: "none",
        margin: 0,
        paddingLeft: "1rem",
        fontFamily: "monospace",
      }}
    >
      <li>
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: "0.5rem",
            cursor: "pointer",
            padding: "2px 4px",
            borderRadius: "4px",
          }}
          onClick={(e) => {
            e.stopPropagation();

            if (hasChildren) {
              setExpanded(!expanded);
            }

            onSelect?.(node);
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.background =
              "rgba(255,255,255,0.08)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.background =
              "transparent";
          }}
        >
          <span
            style={{
              width: "1rem",
              userSelect: "none",
              color: "#888",
            }}
          >
            {hasChildren
              ? expanded
                ? "▼"
                : "▶"
              : "•"}
          </span>

          <span
            style={{
              color: nodeColor(
                node.label
              ),
            }}
          >
            {node.label}
          </span>

          {node.span && (
            <span
              style={{
                color: "#777",
                fontSize: "0.8em",
              }}
            >
              [
              {
                node.span.start_line
              }
              :
              {
                node.span.start_column
              }
              -
              {node.span.end_line}
              :
              {
                node.span.end_column
              }
              ]
            </span>
          )}
        </div>

        {expanded &&
          hasChildren && (
            <div
              style={{
                marginLeft: "0.5rem",
                borderLeft:
                  "1px solid #444",
                paddingLeft: "0.75rem",
              }}
            >
              {node.children.map(
                (
                  child,
                  idx
                ) => (
                  <AstTree
                    key={idx}
                    node={child}
                    onSelect={
                      onSelect
                    }
                  />
                )
              )}
            </div>
          )}
      </li>
    </ul>
  );
}
