import { EXAMPLES } from "../constants";

type Props = {
  onSelect(code: string): void;
};

export function ExampleSelector({
  onSelect,
}: Props) {
  const groupedExamples =
    Object.entries(EXAMPLES).reduce(
      (acc, [key, example]) => {
        const category =
          example.category;

        if (!acc[category]) {
          acc[category] = [];
        }

        acc[category].push({
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
    <select
      defaultValue=""
      onChange={(e) => {
        const key = e.target.value;

        if (key in EXAMPLES) {
          onSelect(
            EXAMPLES[
              key as keyof typeof EXAMPLES
            ].code
          );
        }
      }}
    >

      {Object.entries(
        groupedExamples
      ).map(
        ([category, examples]) => (
          <optgroup
            key={category}
            label={category}
          >
            {examples.map(
              ({
                key,
                example,
              }) => (
                <option
                  key={key}
                  value={key}
                >
                  {example.name}
                </option>
              )
            )}
          </optgroup>
        )
      )}
    </select>
  );
}
