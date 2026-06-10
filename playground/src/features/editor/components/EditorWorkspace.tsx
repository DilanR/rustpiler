import { CodeEditor } from "./CodeEditor";
import { EditorTabs } from "./EditorTabs";
import { OutputEditor } from "./OutputEditor";
import { CompilerTerminal } from "./CompilerTerminal";
import { RunButton } from "./RunButton.tsx";

import { useEditorTabs }
  from "@/features/editor/hooks/useEditorTabs";

import type {
  Diagnostic,
  Range,
} from "@/types";
import { ExampleSelector } from "@/features/examples/index.ts";

type Props = {
  code: string;
  setCode: (
    value: string
  ) => void;

  onRun: () => void;
  loading: boolean;

  diagnostics?: Diagnostic[];
  highlightedRange?: Range;
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
  stdout = "",
  result = "",
}: Props) {
  const {
    activeTab,
    setActiveTab,
  } = useEditorTabs();

  return (
    <div className="editor-workspace">

      <div className="editor-toolbar" >
        <ExampleSelector onSelect={setCode} />

        <EditorTabs
          activeTab={activeTab}
          onChange={setActiveTab}
        />
        <RunButton
          loading={loading}
          onRun={onRun}
        />
      </div >

      <div className="main-container">
        {activeTab ===
          "source" && (
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
            />
          )
        }

        {
          activeTab ===
          "ast" && (
            <OutputEditor
              stdout="AST tab coming soon"
              result=""
            />
          )
        }

        {
          activeTab ===
          "typecheck" && (
            <OutputEditor
              stdout="Typecheck tab coming soon"
              result=""
            />
          )
        }
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
