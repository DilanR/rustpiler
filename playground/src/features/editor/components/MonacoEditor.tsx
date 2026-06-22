import Editor from "@monaco-editor/react";
import type * as monaco from "monaco-editor";
import { MONACO_OPTIONS } from "../constants"
import "../index.css"

type Props = {
  code: string;
  setCode: (value: string) => void;
  onRun?: () => void;
  onSourceSelect?: (
    line: number,
    column: number
  ) => void;

  onMount: (
    editor: monaco.editor.IStandaloneCodeEditor,
    monacoInstance: typeof monaco
  ) => void;
};

export function MonacoEditor({
  code,
  setCode,
  onRun,
  onSourceSelect,
  onMount,
}: Props) {
  function handleMount(
    editor: monaco.editor.IStandaloneCodeEditor,
    monacoInstance: typeof monaco
  ) {
    monacoInstance.editor.setTheme("vs-dark");

    editor.addCommand(
      monacoInstance.KeyMod.CtrlCmd |
      monacoInstance.KeyCode.Enter,
      () => {
        onRun?.();
      }
    );

    editor.onMouseDown((event) => {
      const position =
        event.target.position;

      if (!position) {
        return;
      }

      onSourceSelect?.(
        position.lineNumber,
        position.column - 1
      );
    });

    onMount(editor, monacoInstance);
  }

  return (
    <Editor
      className="editor-terminal"
      language="rust"
      value={code}
      onChange={(value) =>
        setCode(value ?? "")
      }
      onMount={handleMount}
      options={MONACO_OPTIONS}
    />
  );
}
