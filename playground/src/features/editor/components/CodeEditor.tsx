import { useRef } from "react";
import type * as monaco from "monaco-editor";

import { MonacoEditor } from "./MonacoEditor";

import { useMonacoMarkers }
  from "../hooks/useMonacoMarkers";

import { useMonacoDecorations }
  from "../hooks/useMonacoDecorations";

import type {
  Diagnostic,
  Range,
} from "@/types";

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

export function CodeEditor({
  code,
  setCode,
  onRun,
  diagnostics = [],
  highlightedRange,
  onSourceSelect,
}: Props) {
  const editorRef =
    useRef<monaco.editor.IStandaloneCodeEditor | null>(
      null
    );

  const monacoRef =
    useRef<typeof monaco | null>(
      null
    );

  useMonacoMarkers(
    editorRef,
    monacoRef,
    diagnostics
  );

  useMonacoDecorations(
    editorRef,
    monacoRef,
    highlightedRange
  );

  return (
    <MonacoEditor
      code={code}
      setCode={setCode}
      onRun={onRun}
      onSourceSelect={
        onSourceSelect
      }
      onMount={(
        editor,
        monaco
      ) => {
        editorRef.current =
          editor;
        monacoRef.current =
          monaco;
      }}
    />
  );
}
