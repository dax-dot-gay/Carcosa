import Extreme, { param, wildcard } from "extreme-router";
import {
    JSX,
    ReactNode,
    useContext,
    useEffect,
    useMemo,
    useReducer,
    useState,
} from "react";
import {
    RouteContext,
    RouteContextType,
    RouteHandler,
    RouteMatchResult,
    RouteRenderingContext,
    RoutingContext,
    RoutingContextType,
} from "./types";
import { useRouter } from "./utils";
import { trimEnd, trimStart, uniqueId } from "lodash";
import { useDeepCompareEffect, useDeepCompareMemo } from "use-deep-compare";
import { useListState, useStateHistory } from "@mantine/hooks";

type RouterUpdateAction =
    | {
          action: "register";
          path: string;
          context: RouteContextType;
      }
    | {
          action: "deregister";
          path: string;
      };

type RouterState = [string, Extreme<{ context: RouteContextType }>];

export function LocalRouter({
    routerId,
    children,
    initialPath,
    fallback,
}: {
    routerId: string;
    children?: ReactNode | ReactNode[];
    initialPath?: string;
    fallback?: RouteHandler;
}) {
    const parent = useRouter();
    const [router, updateRouter] = useReducer(
        (previous_state: RouterState, action: RouterUpdateAction) => {
            switch (action.action) {
                case "register":
                    previous_state[1].register(action.path).context =
                        action.context;
                    return [uniqueId(), previous_state[1]] as RouterState;
                case "deregister":
                    previous_state[1].unregister(action.path);
                    return [uniqueId(), previous_state[1]] as RouterState;
            }
        },
        [
            uniqueId(),
            new Extreme<{ context: RouteContextType }>({
                plugins: [param, wildcard],
            }),
        ],
    );
    const [path, { set: setPath, ...historyHandlers }, history] =
        useStateHistory(initialPath ?? "/");
    const location: RouteMatchResult = useMemo(() => {
        let matched = router[1].match(path);
        if (matched) {
            return {
                matched: true,
                path,
                params: matched.params,
                context: matched.context,
            } as RouteMatchResult;
        } else {
            return {
                matched: false,
                path,
            };
        }
    }, [router[0], path]);

    const parents = useDeepCompareMemo(() => {
        if (parent) {
            return { ...parent.parents, [parent.id]: parent };
        } else {
            return {};
        }
    }, [parent]);

    const FallbackElement = fallback ?? (() => <></>);
    const methods: RoutingContextType["methods"] = useDeepCompareMemo(() => {
        return {
            registerRoute: (path, handler) =>
                updateRouter({
                    action: "register",
                    path,
                    context: handler,
                }),
            deregisterRoute: (path) =>
                updateRouter({ action: "deregister", path }),
            navigate: (path) => {
                setPath(path);
                let matched = router[1].match(path);
                if (matched) {
                    return {
                        matched: true,
                        path,
                        params: matched.params,
                        context: matched.context,
                    } as RouteMatchResult;
                } else {
                    return {
                        matched: false,
                        path,
                    };
                }
            },
            history: historyHandlers,
        };
    }, [updateRouter]);

    return (
        <RoutingContext.Provider
            value={{
                id: routerId,
                parents,
                location,
                methods,
                history,
            }}
        >
            {children}

            {location.matched ? (
                <RouteRenderingContext.Provider
                    value={{
                        current:
                            location.context.handlerStack[0] ?? FallbackElement,
                        children: location.context.handlerStack.slice(1),
                    }}
                >
                    <Outlet />
                </RouteRenderingContext.Provider>
            ) : (
                <RouteRenderingContext.Provider
                    value={{ current: FallbackElement, children: [] }}
                >
                    <FallbackElement />
                </RouteRenderingContext.Provider>
            )}
        </RoutingContext.Provider>
    );
}

export function Route({
    path,
    element,
    children,
}: {
    path: string;
    element: RouteHandler;
    children?: ReactNode | ReactNode[];
}) {
    const parent = useRouter();
    const parent_route = useContext(RouteContext);

    const routeContext: RouteContextType = useDeepCompareMemo(
        () =>
            parent
                ? {
                      routerId: parent.id,
                      path: parent_route
                          ? trimEnd(parent_route.path, "/") +
                            "/" +
                            trimStart(path, "/")
                          : "/" + trimStart(path, "/"),
                      handlerStack: [
                          ...(parent_route?.handlerStack ?? []),
                          element,
                      ],
                  }
                : (null as any),
        [parent, parent_route, element],
    );

    let [register, deregister] = [
        parent?.methods.registerRoute,
        parent?.methods.deregisterRoute,
    ];

    useDeepCompareEffect(() => {
        if (
            register &&
            deregister &&
            !("/" + trimStart(path, "/") === "/" && children)
        ) {
            register("/" + trimStart(path, "/"), routeContext);
            return () => deregister("/" + trimStart(path, "/"));
        }
    }, [path, routeContext, register, deregister]);
    return parent ? (
        <RouteContext.Provider value={routeContext}>
            {children}
        </RouteContext.Provider>
    ) : (
        <></>
    );
}

export function Outlet({
    passthrough,
}: {
    passthrough?: { [key: string]: any };
}) {
    const rendering = useContext(RouteRenderingContext);
    const [new_current, new_children] = useDeepCompareMemo(() => {
        return [rendering.children[0] ?? null, rendering.children.slice(1)];
    }, [rendering.children]);
    const CurrentRenderer = rendering.current;

    return new_current ? (
        <RouteRenderingContext.Provider
            value={{ current: new_current, children: new_children }}
        >
            <CurrentRenderer {...(passthrough ?? {})} />
        </RouteRenderingContext.Provider>
    ) : (
        <CurrentRenderer {...(passthrough ?? {})} />
    );
}
