import { useState } from "react";
import { EXAMPLES } from "../constants";

import "./example-selector.css";

type Props = {
  onSelect(code: string): void;
};

export function ExampleSelector({
  onSelect,
}: Props) {
  const [open, setOpen] =
    useState(false);

  const [
    hoveredCategory,
    setHoveredCategory,
  ] = useState<string | null>(
    null
  );

  const grouped =
    Object.entries(EXAMPLES).reduce(
      (acc, [key, example]) => {
        if (
          !acc[
          example.category
          ]
        ) {
          acc[
            example.category
          ] = [];
        }

        acc[
          example.category
        ].push({
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

  return (
    <div
      className="examples-menu"
      onMouseLeave={() => {
        setOpen(false);
        setHoveredCategory(null);
      }}
    >
      <button
        className="examples-button"
        onMouseEnter={() =>
          setOpen(true)
        }
      >
        Examples
      </button>

      {open && (
        <div className="examples-dropdown">
          {Object.entries(grouped).map(
            ([category, examples]) => (
              <div
                key={category}
                className="examples-category-container"
                onMouseEnter={() =>
                  setHoveredCategory(
                    category
                  )
                }
              >
                <div className="examples-category">
                  {category} ▶
                </div>

                {hoveredCategory ===
                  category && (
                    <div className="examples-submenu">
                      {examples.map(
                        ({
                          key,
                          example,
                        }) => (
                          <button
                            key={key}
                            className="examples-item"
                            onClick={() => {
                              onSelect(
                                example.code
                              );
                              setOpen(
                                false
                              );
                            }}
                          >
                            {
                              example.name
                            }
                          </button>
                        )
                      )}
                    </div>
                  )}
              </div>
            )
          )}
        </div>
      )}
    </div>
  );
}
