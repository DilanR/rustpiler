import { useWorkSpaceComponents } from "@/workspace/hooks/useWorkSpaceComponents";
import { Footer } from "../Footer";
import { WorkSpace } from "../workspace/components/WorkSpace"

export default function App() {
  return <div className="h-dvh overflow-hidden flex flex-col">
    <WorkSpace {...useWorkSpaceComponents()} />
    <Footer />
  </div>;
}
