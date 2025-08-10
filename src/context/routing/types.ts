import { JSX } from "react";

export type RouteMatchResult<
    T extends { [key: string]: string } = { [key: string]: string },
> =
    | {
          matched: true;
          path: string;
          params: { [key: string]: string };
      }
    | {
          matched: false;
      };

export type RoutingContextType = {
    id: string;
    parents: { [key: string]: RoutingContextType };
    location: RouteMatchResult;
    history: string[];
    methods: {
        registerRoute: (path: string, handler: RouteHandler) => void;
        deregisterRoute: (path: string) => void;
        navigate: <
            T extends { [key: string]: string } = { [key: string]: string },
        >(
            path: string,
        ) => RouteMatchResult<T>;
    };
} | null;

export type RouteHandlerProps = Partial<{
    path: string;
    params: { [key: string]: string };
}>;

export type RouteHandler = <T extends RouteHandlerProps>(
    props?: T,
) => JSX.Element;

export type RouteContextType = {
    path: string;
    routerId: string;
} | null;
