import "../index.css";

import type { AstNode } from "@/types";
import { AstTreeNode } from "./AstTreeNode";

type Props = {
  node?: AstNode;
  onSelect?: (node: AstNode) => void;
};

export function AstTree({ node, onSelect }: Props) {
  if (!node) {
    return <>no ast</>;
  }

  return (
    <AstTreeNode
      node={node}
      onSelect={onSelect}
    />
  );
}
