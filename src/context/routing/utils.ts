import { useContext } from "react";
import { RouteMatchResult, RoutingContext, RoutingContextType } from "./types";
import { useDeepCompareMemo } from "use-deep-compare";
import { UseStateHistoryValue } from "@mantine/hooks";

export function useRouter(id?: string): RoutingContextType | null {
    const parent_router = useContext(RoutingContext);
    const selected = useDeepCompareMemo(() => {
        return parent_router
            ? id
                ? (parent_router.parents[id] ?? null)
                : parent_router
            : null;
    }, [id, parent_router]);
    return selected;
}

export function useNavigate(id?: string): (path: string) => RouteMatchResult {
    const router = useRouter(id);
    return router
        ? router.methods.navigate
        : (path) => ({ matched: false, path });
}

export function useMatch(id?: string): RouteMatchResult {
    const router = useRouter(id);
    return router ? router.location : { matched: false, path: "NO_ROUTER" };
}

export function useLocation(id?: string): string {
    const router = useRouter(id);
    return router ? router.location.path : "NO_ROUTER";
}

export function useParams<
    T extends { [key: string]: string } = { [key: string]: string },
>(id?: string): T | null {
    const match = useMatch(id);
    return match.matched ? (match.params as any as T) : null;
}

export function useHistory(
    id?: string,
): [UseStateHistoryValue<string>, RoutingContextType["methods"]["history"]] {
    const router = useRouter(id);
    return router
        ? [router.history, router.methods.history]
        : [
              { history: [], current: 0 },
              { back: () => {}, forward: () => {}, reset: () => {} },
          ];
}
