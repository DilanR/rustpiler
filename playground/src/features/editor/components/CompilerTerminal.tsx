import Editor from "@monaco-editor/react";
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
      height="150px"
      language="shell"
      value={content}
      options={{
        readOnly: true,
        minimap: {
          enabled: false,
        },
        lineNumbers: "off",
        folding: false,
        scrollBeyondLastLine: false,
        wordWrap: "on",
      }}
    />
  );
}
