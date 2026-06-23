import type { Diagnostic } from "@/types";
import { DiagnosticItem } from "./DiagnosticItem";

type Props = {
  diagnostics: Diagnostic[];
};

export function DiagnosticsView({
  diagnostics,
}: Props) {
  return (
    <>
      {diagnostics.map((diagnostic, index) => (
        <DiagnosticItem
          key={index}
          diagnostic={diagnostic}
        />
      ))}
    </>
  );
}
