export const MONACO_OPTIONS = {
  minimap: {
    enabled: false,
  },
  fontSize: 14,
  automaticLayout: true,
  scrollBeyondLastLine: false,

  scrollbar: {
    vertical: "auto" as const,
    horizontal: "auto" as const,
  },
};

export const MONACO_OPTIONS_READ_ONLY = {
  readOnly: true,
  minimap: {
    enabled: false,
  },
  lineNumbers: "off" as const,
  folding: false,
  scrollBeyondLastLine: false,
  wordWrap: "on" as const,
};
