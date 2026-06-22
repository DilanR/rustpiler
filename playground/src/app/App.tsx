import { useWorkspaceComponents } from "@/workspace/hooks/useWorkspaceComponents";
import { Footer } from "../Footer";
import { Workspace } from "../workspace/components/Workspace"

export default function App() {
  return <>
    <Workspace {...useWorkspaceComponents()} />
    <Footer />
  </>;
}
