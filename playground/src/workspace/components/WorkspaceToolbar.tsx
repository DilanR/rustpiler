import { ExampleSelector } from "@/features/examples";
import { RunButton } from "@/features/editor"

type Props = {
  onSelect: (
    value: string
  ) => void;
  loading: boolean;
  onRun: () => void;
};

export function WorkspaceToolbar({ loading, onRun, onSelect }: Props) {
  return (
    <div className="editor-toolbar" >
      <ExampleSelector onSelect={onSelect} />

      <RunButton
        loading={loading}
        onRun={onRun}
      />
    </div >
  );
}
