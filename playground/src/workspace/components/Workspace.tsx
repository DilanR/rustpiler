import type { WorkspaceComponents } from "@/types";
import "../index.css"

/*
┌─────────────────────────────────────────────────────────────────────┐
│ WorkSpaceToolbar                                                    │
├─────────────────────────────────────────────────────────────────────┤
│ Content                                          Sidebar            │
│  ┌──────────────────────────────────┐ ┌──────────────────────────┐  │
│  │                                  │ │SidebarToolbar            │  │
│  │             Editor               │ │──────────────────────────│  │
│  │                                  │ │                          │  │
│  │                                  │ │                          │  │
│  │                                  │ │                          │  │
│  │                                  │ │                          │  │
│  └──────────────────────────────────┘ └──────────────────────────┘  │
│                                                                     │
├─────────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────────┐ │
│ │ Console                                                         │ │
│ │                                                                 │ │
│ │                                                                 │ │
│ │                                                                 │ │
│ │                                                                 │ │
│ │                                                                 │ │
│ └─────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
*/

export function Workspace(
  components:
    WorkspaceComponents) {
  return (
    <div className="layout">
      <header className="toolbar">
        {components.toolbar}
      </header>

      <section className="content">

        <div className="main">
          {components.editor}
        </div>

        <aside className="sidebar">
          {components.sidebar}
        </aside>

      </section>

      <footer className="output">
        {components.console}
      </footer>

    </div>
  );
}
