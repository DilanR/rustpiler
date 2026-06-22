import "../index.css";

type Props = {
  loading: boolean;
  onRun: () => void;
};

export function RunButton({ loading, onRun }: Props) {
  return (
    <button className="run-button" onClick={onRun} disabled={!loading}>
      {!loading ? "Loading..." : "▶ Run"}
    </button>
  );
}
