import Extreme, { param, wildcard } from "extreme-router";
import { JSX, ReactNode, useReducer } from "react";
import { RouteHandler, RouteHandlerProps } from "./types";

type RouterUpdateAction =
    | {
          action: "register";
          path: string;
          handler: RouteHandler;
      }
    | {
          action: "deregister";
          path: string;
      };

export function LocalRouter({
    routerId,
    children,
    index,
    fallback,
}: {
    routerId: string;
    children?: ReactNode | ReactNode[];
    index?: string;
    fallback?: RouteHandler;
}) {
    const [router, updateRouter] = useReducer(
        (previous_state, action: RouterUpdateAction) => {
            switch (action.action) {
                case "register":
                    previous_state.register(action.path).handler =
                        action.handler;
                    return previous_state;
                case "deregister":
                    previous_state.unregister(action.path);
                    return previous_state;
            }
        },
        new Extreme<{
            handler: RouteHandler;
        }>({
            plugins: [param, wildcard],
        }),
    );
}
