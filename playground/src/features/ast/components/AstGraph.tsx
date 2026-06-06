import { useEffect, useMemo, useRef, useState } from "react";
import * as d3 from "d3";
import type { AstNode } from "@/types";

type Props = {
  node?: AstNode;
  onSelect?: (node: AstNode) => void;
};

function buildVisibleTree(
  node: AstNode,
  expanded: Set<string>,
  path = "root"
): AstNode {
  return {
    ...node,
    children: expanded.has(path)
      ? node.children.map((child, i) =>
        buildVisibleTree(
          child,
          expanded,
          `${path}.${i}`
        )
      )
      : [],
  };
}

export function AstGraph({
  node,
  onSelect,
}: Props) {
  const svgRef =
    useRef<SVGSVGElement>(null);

  const [expanded, setExpanded] =
    useState<Set<string>>(
      new Set(["root"])
    );

  const visibleTree = useMemo(() => {
    if (!node) {
      return undefined;
    }

    return buildVisibleTree(
      node,
      expanded
    );
  }, [node, expanded]);

  useEffect(() => {
    if (
      !visibleTree ||
      !svgRef.current
    ) {
      return;
    }

    const svg =
      d3.select(svgRef.current);

    svg.selectAll("*").remove();

    const root =
      d3.hierarchy(visibleTree);

    const tree =
      d3
        .tree<AstNode>()
        .nodeSize([180, 100]);

    tree(root);

    const width = 1400;
    const height =
      Math.max(
        800,
        root.height * 120
      );

    svg
      .attr("width", width)
      .attr("height", height);

    const g = svg
      .append("g")
      .attr(
        "transform",
        "translate(80,50)"
      );

    //
    // Links
    //

    g.selectAll(".link")
      .data(root.links() as d3.HierarchyPointLink<AstNode>[])
      .enter()
      .append("path")
      .attr(
        "fill",
        "none"
      )
      .attr(
        "stroke",
        "#666"
      )
      .attr(
        "stroke-width",
        1.5
      )
      .attr(
        "d",
        d3
          .linkVertical<
            d3.HierarchyPointLink<AstNode>,
            d3.HierarchyPointNode<AstNode>
          >()
          .x(
            (d) => d.x
          )
          .y(
            (d) => d.y
          )
      );

    //
    // Nodes
    //

    const nodes = g
      .selectAll(".node")
      .data(
        root.descendants()
      )
      .enter()
      .append("g")
      .attr(
        "transform",
        (d) =>
          `translate(${d.x},${d.y})`
      )
      .style(
        "cursor",
        "pointer"
      );

    nodes
      .append("rect")
      .attr("x", -60)
      .attr("y", -15)
      .attr("width", 120)
      .attr("height", 30)
      .attr("rx", 4)
      .attr(
        "fill",
        "#282c34"
      )
      .attr(
        "stroke",
        "#61afef"
      );

    nodes
      .append("text")
      .text(
        (d) => d.data.label
      )
      .attr(
        "text-anchor",
        "middle"
      )
      .attr("dy", "0.35em")
      .attr(
        "fill",
        "white"
      )
      .style(
        "font-family",
        "monospace"
      )
      .style(
        "font-size",
        "12px"
      );

    nodes.on(
      "click",
      (_, d) => {
        const path =
          d
            .ancestors()
            .reverse()
            .slice(1)
            .map((n) =>
              n.parent
                ? n.parent.children?.indexOf(
                  n
                )
                : undefined
            )
            .filter(
              (
                x
              ): x is number =>
                x !==
                undefined
            )
            .reduce(
              (acc, i) =>
                `${acc}.${i}`,
              "root"
            );

        setExpanded(
          (
            prev
          ) => {
            const next =
              new Set(prev);

            if (
              next.has(path)
            ) {
              next.delete(
                path
              );
            } else {
              next.add(
                path
              );
            }

            return next;
          }
        );

        onSelect?.(
          d.data
        );
      }
    );
  }, [
    visibleTree,
    onSelect,
  ]);

  if (!node) {
    return null;
  }

  return (
    <svg
      ref={svgRef}
      style={{
        width: "100%",
        border:
          "1px solid #444",
        background:
          "#1e1e1e",
      }}
    />
  );
}
