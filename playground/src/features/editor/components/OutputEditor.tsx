import Editor from "@monaco-editor/react";
import { EDITOR_HEIGHT, MONACO_OPTIONS } from "../constants"

type Props = {
  stdout?: string;
  result?: string;
};

export function OutputEditor({
  stdout = "",
  result = "",
}: Props) {
  const content = [
    "STDOUT",
    "======",
    stdout || "<empty>",
    "",
    "RESULT",
    "======",
    result || "<empty>",
  ].join("\n");

  return (
    <Editor
      height={EDITOR_HEIGHT}
      language="plaintext"
      value={content}
      options={MONACO_OPTIONS}
    />
  );
}
