import type { AstNode } from "@/types";
import * as d3 from "d3";
import { useEffect } from "react";

export function useD3AstGraph(
  svgRef: React.RefObject<SVGSVGElement | null>,
  visibleTree?: AstNode,
  onNodeClick?: (
    node: AstNode,
    path: string
  ) => void
) {
  useEffect(() => {
    if (
      !visibleTree ||
      !svgRef.current
    ) {
      return;
    }

    const svg = d3.select(svgRef.current);

    svg.selectAll("*").remove();

    const root =
      d3.hierarchy<AstNode>(visibleTree);

    const tree = d3
      .tree<AstNode>()
      .nodeSize([140, 80]);

    tree(root);

    const g = svg
      .append("g")
      .attr(
        "transform",
        "translate(190,50)"
      );

    //
    // Links
    //

    g.selectAll(".link")
      .data(
        root.links() as d3.HierarchyPointLink<AstNode>[]
      )
      .enter()
      .append("path")
      .attr("fill", "none")
      .attr("stroke", "#666")
      .attr("stroke-width", 1.5)
      .attr(
        "d",
        d3
          .linkVertical<
            d3.HierarchyPointLink<AstNode>,
            d3.HierarchyPointNode<AstNode>
          >()
          .x((d) => d.x)
          .y((d) => d.y)
      );

    //
    // Nodes
    //

    const nodes = g
      .selectAll(".node")
      .data(root.descendants())
      .enter()
      .append("g")
      .attr(
        "class",
        "node"
      )
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
        const path = d
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
              x !== undefined
          )
          .reduce(
            (acc, i) =>
              `${acc}.${i}`,
            "root"
          );

        onNodeClick?.(
          d.data,
          path
        );
      }
    );
  }, [
    visibleTree,
    onNodeClick,
    svgRef,
  ]);
}
