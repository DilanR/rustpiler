import "../index.css";
import { useState } from "react";

import type { Diagnostic } from "@/types";
import { severityIcons } from "../utils/icons";

type Props = {
  diagnostic: Diagnostic;
  onSelect?: (range: Range) => void;
};

export function DiagnosticItem({
  diagnostic,
}: Props) {
  const [collapsed, setCollapsed] =
    useState(false);

  const Icon =
    severityIcons[diagnostic.severity];

  //TODO: onClick -> setHighlighted for source code
  return (
    <div className="diagnostic">

      <div
        className="diagnostic__header"
        onClick={() =>
          setCollapsed(!collapsed)
        }
      >
        <div className="diagnostic__severity">
          <Icon />
          {/*Render amount of related on collapsed*/}
          {collapsed &&
            diagnostic.related.length > 0 && (
              <div className="diagnostic__related--hint">{diagnostic.related.length}</div>
            )
          }
        </div>

        <span>
          {diagnostic.message}
        </span>
      </div>

      {
        !collapsed &&
        diagnostic.related.length > 0 && (
          <div className="diagnostic__related">

            {diagnostic.related.map(
              (related, index) => (
                <DiagnosticItem
                  key={index}
                  diagnostic={related}
                />
              )
            )}

          </div>
        )
      }

    </div >
  );
}
