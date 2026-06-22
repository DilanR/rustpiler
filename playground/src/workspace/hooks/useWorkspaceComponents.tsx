import { Editor } from "../components/Editor";
import { useWorkspace } from "./useWorkspace";
import { Sidebar } from "../components/Sidebar";
import { WorkspaceToolbar } from "../components/WorkspaceToolbar";
import { Console } from "../components/Console";
import type { WorkspaceComponents } from "@/types";

export function useWorkspaceComponents(): WorkspaceComponents {
  const workspace = useWorkspace();

  return {
    toolbar: <WorkspaceToolbar {...workspace.toolBarState} />,
    editor: <Editor {...workspace.editorState} />,
    sidebar: <Sidebar {...workspace.sideBarState} />,
    console: <Console {...workspace.consoleState} />,
  };
}
