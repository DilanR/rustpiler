import type { ReactNode } from "react";

export type DiagnosticSeverity =
  | "Error"
  | "Warning"
  | "Info";

export type Range = {
  start_line: number;
  start_column: number;
  end_line: number;
  end_column: number;
};

export type Diagnostic = {
  message: string;
  severity: DiagnosticSeverity;
  range: Range;
  related: Diagnostic[];
};

export type AstNode = {
  kind: string;
  label: string;
  span?: Range;
  children: AstNode[];
};

export type CompileResult = {
  diagnostics: Diagnostic[];
  result?: string;
  stdout?: string;
  ast?: AstNode;
  time_ms?: number;
};

export type ExampleCategory =
  | "Features"
  | "Diagnostics"
  | "Programs";

export type Example = {
  id: string;
  category: ExampleCategory;
  name: string;
  description: string;
  code: string;
};

export type EditorTab =
  | "source"
  | "ast"
  | "typecheck";

export type WorkspaceComponents = {
  toolbar: ReactNode;
  editor: ReactNode;
  sidebar: ReactNode;
  console: ReactNode;
}
