import { ExampleSelector } from "@/features/examples";
import { RunButton } from "@/features/editor/components/RunButton"

type Props = {
  onSelect: (
    value: string
  ) => void;
  loading: boolean;
  onRun: () => void;
};

export function ToolBar({ loading, onRun, onSelect }: Props) {
  return (
    <div className="editor-toolbar pt-4 px-4" >
      <ExampleSelector onSelect={onSelect} />

      <RunButton
        loading={loading}
        onRun={onRun}
      />
    </div >
  );
}
