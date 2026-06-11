import { useEffect, useState } from "react";
import init, { compile } from "../../../../pkg";
import type { CompileResult } from "@/types";

export function useRustpiler() {
  const [ready, setReady] = useState(false);

  const [result, setResult] =
    useState<CompileResult | null>(null);

  const [code, setCode] = useState(`fn main() -> i32 {
    let mut x = 1;
    x = x + 1;
    x
  }`);

  useEffect(() => {
    init()
      .then(() => {
        setReady(true);
      })
      .catch((err) => {
        console.error("Failed to initialize WASM", err);
      });
  }, []);

  const run = () => {
    if (!ready) {
      return;
    }

    try {
      const response = compile(code) as CompileResult;
      console.log(response);
      setResult(response);
    } catch (err) {
      console.error(err);
      setResult({
        diagnostics: [
          {
            message: String(err),
            severity: "Error",
            range: {
              start_line: 1,
              start_column: 1,
              end_line: 1,
              end_column: 1,
            },
            related: []
          },
        ],
      });
    }
  };

  return {
    ready,
    code,
    setCode,
    result,
    diagnostics:
      result?.diagnostics ?? [],
    output:
      result?.result ?? "",
    run,
  };
}
