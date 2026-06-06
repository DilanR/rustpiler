import { useRustpiler } from "@/features/compiler";
import { CodeEditor } from "@/features/editor";
import { AstGraph } from "@/features/ast";
import { RunButton } from "@/features/compiler";
import { ExampleSelector } from "@/features/examples";
import { AstTree } from "@/features/ast";
import { useEffect, useState } from "react";
import type { Range } from "@/types";

export function RustpilerPage() {

  const rustpiler = useRustpiler();
  const [selectedRange, setSelectedRange] =
    useState<Range>();

  useEffect(() => {
    console.log(selectedRange);
  }, [selectedRange]);
  return (
    <div style={{ padding: 20 }}>
      <h2>RnR Compiler</h2>

      <ExampleSelector
        onSelect={rustpiler.setCode}
      />

      <CodeEditor
        code={rustpiler.code}
        setCode={rustpiler.setCode}
        onRun={rustpiler.run}
        diagnostics={rustpiler.diagnostics}
        highlightedRange={selectedRange}
      />

      <RunButton
        loading={false}
        onRun={rustpiler.run}
      />

      <AstTree
        node={rustpiler.result?.ast}
        onSelect={(node) => {
          console.log(node);

          if (node.span) {
            setSelectedRange(node.span);
          }
        }}
      />


      <AstGraph
        node={rustpiler.result?.ast}
        onSelect={(node) => {
          if (node.span) {
            setSelectedRange(node.span);
          }
        }}
      />

    </div>
  );
}
