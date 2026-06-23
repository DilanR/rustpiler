import { CompilerTerminal } from "@/features/editor";
import "../index.css"

type Props = {
  time_ms?: number;
  stdout: string;
  result: string;
};

function formatTime(ms: number): string {
  if (ms < 1) {
    return "< 1 ms"
  } else {
    return ms + " ms"
  }
}

export function Console({
  time_ms,
  stdout,
  result,
}: Props) {
  return (
    <>
      <div className="container-legend">
        Console
        {time_ms !== undefined && (
          <span className="container-legend__time">
            {formatTime(time_ms)}
          </span>
        )}
      </div>
      <CompilerTerminal
        stdout={stdout}
        result={result}
      />

    </>
  );
}
