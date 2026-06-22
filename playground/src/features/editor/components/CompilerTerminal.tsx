import Editor from "@monaco-editor/react";
import { MONACO_OPTIONS_READ_ONLY } from "../constants"
import "../index.css"

type Props = {
  stdout?: string;
  result?: string;
};

export function CompilerTerminal({
  stdout = "",
  result = "",
}: Props) {
  const content = [
    stdout,
    `${result}`,
  ].join("\n");

  return (
    <Editor
      className="output-terminal"
      language="shell"
      value={content}
      options={MONACO_OPTIONS_READ_ONLY}
    />
  );
}
