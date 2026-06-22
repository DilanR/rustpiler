import type { SidebarView } from "./Sidebar";

type Props = {
  collapsed: boolean;
  onToggle: () => void;
  view: SidebarView;
  onChange: (view: SidebarView) => void;

};

export function SidebarToolbar({
  collapsed,
  onToggle,
  view,
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
        type="button"
        className={
          view === "ast"
            ? "sidebar-toolbar__button sidebar-toolbar__button--active"
            : "sidebar-toolbar__button"
        }
        onClick={() => onChange("ast")}
      >
        AST
      </button>

      <button
        type="button"
        className="sidebar-toolbar__button sidebar-toolbar__button--disabled"
        disabled
      >
        Diagnostics
      </button>

      <button
        type="button"
        className="sidebar-toolbar__button sidebar-toolbar__button--disabled"
        disabled
      >
        MIPS ASM
      </button>
    </div>
  );
}
