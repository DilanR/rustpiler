// hooks/useExampleSelector.ts
import { useMemo, useState } from "react";
import { EXAMPLES } from "../constants";

export function useExampleSelector(
  onSelect: (code: string) => void
) {
  const [open, setOpen] = useState(false);
  const [hoveredCategory, setHoveredCategory] =
    useState<string | null>(null);

  const grouped = useMemo(() => {
    return Object.entries(EXAMPLES).reduce(
      (acc, [key, example]) => {
        (acc[example.category] ??= []).push({
          key,
          example,
        });

        return acc;
      },
      {} as Record<
        string,
        {
          key: string;
          example: (typeof EXAMPLES)[keyof typeof EXAMPLES];
        }[]
      >
    );
  }, []);

  return {
    open,
    grouped,
    hoveredCategory,

    openMenu: () => setOpen(true),

    closeMenu: () => {
      setOpen(false);
      setHoveredCategory(null);
    },

    hoverCategory: setHoveredCategory,

    selectExample: (code: string) => {
      onSelect(code);
      setOpen(false);
    },
  };
}
