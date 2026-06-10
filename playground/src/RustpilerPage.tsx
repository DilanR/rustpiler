import { useRustpiler } from "@/features/compiler";
import { useEffect, useState } from "react";
import type { Range } from "@/types";
import { EditorWorkspace } from "./features/editor/components/EditorWorkspace";

export function RustpilerPage() {

  const rustpiler = useRustpiler();
  const [selectedRange/*, setSelectedRange*/] =
    useState<Range>();

  useEffect(() => {
  }, [selectedRange]);
  return (

    < EditorWorkspace
      code={rustpiler.code}
      setCode={rustpiler.setCode}
      onRun={rustpiler.run}
      loading={rustpiler.ready}
      diagnostics={rustpiler.diagnostics}
      highlightedRange={selectedRange}
      stdout={rustpiler.result?.stdout}
      result={
        String(
          rustpiler.result?.result ??
          ""
        )
      }
    />
  );
}
