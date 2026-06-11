import { CodeEditor } from "./CodeEditor";
import { CompilerTerminal } from "./CompilerTerminal";
import { RunButton } from "./RunButton.tsx";

import type {
  AstNode,
  Diagnostic,
  Range,
} from "@/types";
import { ExampleSelector } from "@/features/examples/index.ts";
import { AstPanel } from "@/features/ast";

type Props = {
  code: string;
  setCode: (
    value: string
  ) => void;

  onRun: () => void;
  loading: boolean;

  diagnostics?: Diagnostic[];
  highlightedRange?: Range;
  ast?: AstNode;
  selectedAstNode?: AstNode;
  onSourceSelect?: (
    line: number,
    column: number
  ) => void;
  onAstSelect?: (node: AstNode) => void;
  onAstReset?: () => void;
  stdout?: string;
  result?: string;
};

export function EditorWorkspace({
  code,
  setCode,
  onRun,
  loading,
  diagnostics = [],
  highlightedRange,
  ast,
  selectedAstNode,
  onSourceSelect,
  onAstSelect,
  onAstReset,
  stdout = "",
  result = "",
}: Props) {
  return (
    <div className="editor-workspace">

      <div className="editor-toolbar" >
        <ExampleSelector onSelect={setCode} />

        <RunButton
          loading={loading}
          onRun={onRun}
        />
      </div >

      <div className="main-container">
        <div className="source-panel">
          <CodeEditor
            code={code}
            setCode={setCode}
            onRun={onRun}
            diagnostics={
              diagnostics
            }
            highlightedRange={
              highlightedRange
            }
            onSourceSelect={
              onSourceSelect
            }
          />
        </div>

        <AstPanel
          root={ast}
          selectedRoot={selectedAstNode}
          onSelect={onAstSelect}
          onReset={onAstReset}
        />
      </div>

      <div className="editor-terminal">
        <CompilerTerminal
          stdout={stdout}
          result={result}
        />
      </div>

    </div>
  );
}
