import { useState } from "react";

export function useExpandedAst() {
  const [expanded, setExpanded] =
    useState(
      new Set(["root"])
    );

  function toggleNode(
    path: string
  ) {
    setExpanded((prev) => {
      const next =
        new Set(prev);

      if (next.has(path)) {
        next.delete(path);
      } else {
        next.add(path);
      }

      return next;
    });
  }

  return {
    expanded,
    toggleNode,
  };
}
