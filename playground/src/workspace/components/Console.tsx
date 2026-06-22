import { CompilerTerminal } from "@/features/editor";
import "../index.css"

type Props = {
  stdout: string;
  result: string;
};

export function Console({
  stdout,
  result,
}: Props) {
  return (
    <>
      <div className="container-legend">
        Console
      </div>
      <CompilerTerminal
        stdout={stdout}
        result={result}
      />

    </>
  );
}
