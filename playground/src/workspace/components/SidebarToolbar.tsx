import type { SidebarView } from "./Sidebar";

type Props = {
  diagnosticsCount: number;
  collapsed: boolean;
  onToggle: () => void;
  onChange: (view: SidebarView) => void;

};

export function SidebarToolbar({
  diagnosticsCount,
  collapsed,
  onToggle,
  onChange,

}: Props) {
  return (

    <div className="sidebar-toolbar">

      <button
        className="sidebar-toolbar__button sidebar-toolbar__button--compact"
        onClick={onToggle}
      >
        {collapsed ? "◀" : "▶"}
      </button>
      <button
        className="sidebar-toolbar__button"
        onClick={() => onChange("ast")}
      >
        AST
      </button>

      <button
        className="sidebar-toolbar__button"
        disabled={diagnosticsCount === 0}
        onClick={() => onChange("diagnostics")}
      >
        DIAGNOSTICS {diagnosticsCount > 0 && (<span>{diagnosticsCount}</span>)}
      </button>

      <button
        className="sidebar-toolbar__button"
        disabled={true}
      >
        MIPS ASM
      </button>
    </div >
  );
}
