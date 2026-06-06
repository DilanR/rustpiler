type Props = {
  loading: boolean;
  onRun: () => void;
};

export function RunButton({ loading, onRun }: Props) {
  return (
    <button onClick={onRun} disabled={loading}>
      {loading ? "Running..." : "Run"}
    </button>
  );
}
