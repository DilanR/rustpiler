import type { DiagnosticSeverity } from "@/types";
import {
  FaCircleInfo,
  FaTriangleExclamation,
  FaCircleXmark,
  //FaLightbulb,
} from "react-icons/fa6";

export const severityIcons = {
  Error: FaCircleXmark,
  Warning: FaTriangleExclamation,
  Info: FaCircleInfo,
  //Hint: FaLightbulb,
} satisfies Record<
  DiagnosticSeverity,
  React.ComponentType
>;
