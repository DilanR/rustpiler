import { CodeEditor } from "@/features/editor";
import type { Range, Diagnostic } from "@/types";
import "../index.css"

type Props = {
  code: string;
  setCode: (v: string) => void;
  onRun?: () => void;
  diagnostics?: Diagnostic[];
  highlightedRange?: Range;
  onSourceSelect?: (
    line: number,
    column: number
  ) => void;
};

export function Editor({ code, setCode, onRun, diagnostics, highlightedRange, onSourceSelect }: Props) {
  return (
    <>
      <div className="container-legend">
        Editor
      </div>
      <CodeEditor
        code={code}
        setCode={setCode}
        onRun={onRun}
        diagnostics={diagnostics}
        highlightedRange={highlightedRange}
        onSourceSelect={onSourceSelect}
      />
    </>
  );
}
