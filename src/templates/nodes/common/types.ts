import { Node, NodeDesc, Template } from "@/api";
import { createContext } from "react";

export type NodeEditorContextType = {
    template: Template;
    editable: boolean;
};

export const NodeEditorContext = createContext<NodeEditorContextType>(
    null as any,
);

export type EditableNodeProps<T extends Node> =
    T["node"]["node_category"] extends "container"
        ? {
              node: T;
              parent?: Node;
              children: { [key: string]: Node[] };
          }
        : {
              node: T;
              parent?: Node;
          };

export type EditableContainerProps = {
    parent: Node;
    containerId: string;
    children: Node[];
};

export type TypedNode<
    Category extends Node["node"]["node_category"],
    Kind extends Extract<NodeDesc, { node_category: Category }>["node_kind"],
> = Omit<Node, "node"> & {
    node: Extract<NodeDesc, { node_category: Category; node_kind: Kind }>;
};
