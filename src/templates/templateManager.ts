import { Node, Template } from "@/api";
import { cloneDeep, merge, omit, without } from "lodash";
import { ActionDispatch, useCallback, useEffect, useReducer } from "react";

export type TemplateManagerAction =
    | {
          action: "register";
          node: Node;
      }
    | {
          action: "deregister";
          id: string;
      }
    | {
          action: "place_child";
          id: string;
          index: number;
      }
    | {
          action: "remove_child";
          id: string;
      }
    | {
          action: "set_metadata";
          fields: Partial<
              Pick<
                  Template,
                  "description" | "icon" | "layout" | "inherit" | "name"
              >
          >;
      };

export type UseTemplateManagerReturnType = {
    template: Template;
    modify: ActionDispatch<[action: TemplateManagerAction]>;
    setMeta: <
        T extends keyof Extract<
            TemplateManagerAction,
            {
                action: "set_metadata";
            }
        >["fields"],
    >(
        field: T,
        value: Extract<
            TemplateManagerAction,
            {
                action: "set_metadata";
            }
        >["fields"][T],
    ) => void;
};

export function useTemplateManager(
    template: Template,
): UseTemplateManagerReturnType {
    const [state, dispatchState] = useReducer(
        (previous: Template, action: TemplateManagerAction) => {
            let updated = cloneDeep(previous);
            switch (action.action) {
                case "register":
                    updated.nodes[action.node.id] = action.node;
                    return updated;
                case "deregister":
                    updated.nodes = omit(updated.nodes, action.id);
                    return updated;
                case "place_child":
                    updated.root_children = without(
                        updated.root_children,
                        action.id,
                    );
                    updated.root_children.splice(action.index, 0, action.id);
                    return updated;
                case "remove_child":
                    updated.root_children = without(
                        updated.root_children,
                        action.id,
                    );
                    return updated;
                case "set_metadata":
                    merge(updated, action.fields);
                    return updated;
            }
        },
        cloneDeep(template),
    );
    const setMetadata = useCallback(
        <
            T extends keyof Extract<
                TemplateManagerAction,
                { action: "set_metadata" }
            >["fields"],
        >(
            field: T,
            value: Extract<
                TemplateManagerAction,
                { action: "set_metadata" }
            >["fields"][T],
        ) => {
            dispatchState({
                action: "set_metadata",
                fields: { [field]: value },
            });
        },
        [dispatchState],
    );

    return {
        template: state,
        modify: dispatchState,
        setMeta: setMetadata,
    };
}
