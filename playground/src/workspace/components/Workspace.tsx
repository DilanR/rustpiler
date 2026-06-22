import type { WorkspaceComponents } from "@/types";
import "../index.css"

/*
┌─────────────────────────────────────────────────────────────────────┐
│ Toolbar                                                             │
├─────────────────────────────────────────────────────────────────────┤
│ Content                                                             │
│  ┌──────────────────────────────────┐ ┌──────────────────────────┐  │
│  │                                  │ │                          │  │
│  │             Editor               │ │         Sidebar          │  │
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

export function WorkSpace(
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
