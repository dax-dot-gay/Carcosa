import { UseStateHistoryValue } from "@mantine/hooks";
import { createContext, JSX } from "react";

export type RouteMatchResult =
    | {
          matched: true;
          path: string;
          params: { [key: string]: string };
          context: RouteContextType;
      }
    | {
          matched: false;
          path: string;
      };

export type RoutingContextType = {
    id: string;
    parents: { [key: string]: RoutingContextType };
    location: RouteMatchResult;
    history: UseStateHistoryValue<string>;
    methods: {
        registerRoute: (path: string, handler: RouteContextType) => void;
        deregisterRoute: (path: string) => void;
        navigate: (path: string) => RouteMatchResult;
        history: {
            back: (steps?: number) => void;
            forward: (steps?: number) => void;
            reset: () => void;
        };
    };
};

export const RoutingContext = createContext<RoutingContextType>(null as any);

export type RouteHandler = (props?: any) => JSX.Element;

export type RouteContextType = {
    path: string;
    routerId: string;
    handlerStack: RouteHandler[];
};

export const RouteContext = createContext<RouteContextType>(null as any);

export type RouteRenderingContextType = {
    current: RouteHandler;
    children: RouteHandler[];
};

export const RouteRenderingContext = createContext<RouteRenderingContextType>(
    null as any,
);
