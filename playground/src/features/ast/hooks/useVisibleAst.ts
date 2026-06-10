import { useMemo } from "react";

import type { AstNode }
  from "@/types";

import { buildVisibleTree }
  from "../utils/buildVisibleTree";

export function useVisibleAst(
  node?: AstNode,
  expanded?: Set<string>
) {
  return useMemo(() => {
    if (
      !node ||
      !expanded
    ) {
      return undefined;
    }

    return buildVisibleTree(
      node,
      expanded
    );
  }, [node, expanded]);
}
