import Editor from "@monaco-editor/react";
import type * as monaco from "monaco-editor";
import { EDITOR_HEIGHT, MONACO_OPTIONS } from "../constants"

type Props = {
  code: string;
  setCode: (value: string) => void;
  onRun?: () => void;

  onMount: (
    editor: monaco.editor.IStandaloneCodeEditor,
    monacoInstance: typeof monaco
  ) => void;
};

export function MonacoEditor({
  code,
  setCode,
  onRun,
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

    onMount(editor, monacoInstance);
  }

  return (
    <Editor
      height={EDITOR_HEIGHT}
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
