import { useEffect, useRef } from "react";
import type * as monaco from "monaco-editor";
import type { Range } from "@/types";

export function useMonacoDecorations(
  editorRef: React.RefObject<monaco.editor.IStandaloneCodeEditor | null>,
  monacoRef: React.RefObject<typeof monaco | null>,
  highlightedRange?: Range
) {
  const decorationsCollectionRef =
    useRef<monaco.editor.IEditorDecorationsCollection | null>(
      null
    );

  useEffect(() => {
    if (
      !highlightedRange ||
      !editorRef.current ||
      !monacoRef.current
    ) {
      return;
    }

    const editor = editorRef.current;
    const monaco = monacoRef.current;

    if (!decorationsCollectionRef.current) {
      decorationsCollectionRef.current =
        editor.createDecorationsCollection();
    }

    decorationsCollectionRef.current.set([
      {
        range: new monaco.Range(
          highlightedRange.start_line,
          highlightedRange.start_column + 1,
          highlightedRange.end_line,
          highlightedRange.end_column + 1
        ),
        options: {
          inlineClassName:
            "code-highlight",
        },
      },
    ]);

    editor.revealLineInCenter(
      highlightedRange.start_line
    );
  }, [
    highlightedRange,
    editorRef,
    monacoRef,
  ]);
}
