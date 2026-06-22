import { AstPanel } from "@/features/ast";
import type { AstNode } from "@/types";
type Props = {
  root: AstNode | undefined;
  selectedRoot: AstNode | undefined;
  onSelect: (node: AstNode) => void;
  onReset: () => void;
}

export function SideBar({ root, selectedRoot, onSelect, onReset }: Props) {
  return (
    <AstPanel
      root={root}
      selectedRoot={selectedRoot}
      onSelect={onSelect}
      onReset={onReset}
    />
  );
}
