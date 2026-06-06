import { useEffect } from "react";
import type * as monaco from "monaco-editor";
import type { Diagnostic } from "@/types";

export function useMonacoMarkers(
  editorRef: React.RefObject<monaco.editor.IStandaloneCodeEditor | null>,
  monacoRef: React.RefObject<typeof monaco | null>,
  diagnostics: Diagnostic[]
) {
  useEffect(() => {
    if (
      !editorRef.current ||
      !monacoRef.current
    ) {
      return;
    }

    const model =
      editorRef.current.getModel();

    if (!model) {
      return;
    }

    const monaco =
      monacoRef.current;

    const markers: monaco.editor.IMarkerData[] =
      diagnostics.map((diag) => ({
        startLineNumber:
          diag.range.start_line,
        startColumn:
          diag.range.start_column + 1,
        endLineNumber:
          diag.range.end_line,
        endColumn:
          diag.range.end_column + 1,
        message: diag.message,
        severity:
          diag.severity === "Error"
            ? monaco.MarkerSeverity
              .Error
            : diag.severity ===
              "Warning"
              ? monaco
                .MarkerSeverity
                .Warning
              : monaco
                .MarkerSeverity
                .Info,
      }));

    monaco.editor.setModelMarkers(
      model,
      "rustpiler",
      markers
    );
  }, [
    diagnostics,
    editorRef,
    monacoRef,
  ]);
}
