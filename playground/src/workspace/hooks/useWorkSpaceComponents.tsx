import { Editor } from "../components/Editor";
import { useWorkSpace } from "./useWorkSpace";
import { SideBar } from "../components/SideBar";
import { ToolBar } from "../components/ToolBar";
import { Console } from "../components/Console";
import type { WorkspaceComponents } from "@/types";

export function useWorkSpaceComponents(): WorkspaceComponents {
  const workspace = useWorkSpace();

  return {
    toolbar: <ToolBar {...workspace.toolBarState} />,
    editor: <Editor {...workspace.editorState} />,
    sidebar: <SideBar {...workspace.sideBarState} />,
    console: <Console {...workspace.consoleState} />,
  };
}
