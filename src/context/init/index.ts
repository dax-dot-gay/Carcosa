import { useCallback, useContext, useEffect, useMemo, useState } from "react";
import { InitContext, InitContextType } from "./types";
import api, { State, StateKey } from "@/api";

export function useInit(): InitContextType {
    return useContext(InitContext);
}

export function useAppState(): State {
    return useInit().state;
}

export function useAppStateItem<T extends StateKey>(key: T): State[T] {
    const state = useAppState();
    const updating_val = state[key];
    return useMemo(() => {
        return state[key];
    }, [key, updating_val]);
}

export function usePersistedState<T extends StateKey>(
    key: T
): [State[T], (state: State[T]) => void] {
    const serverside = useAppStateItem(key);
    const [state, setState] = useState(serverside);

    const persistState = useCallback(
        (state: State[T]) => {
            setState(state);
            api.application.set_state({ key, value: state } as any);
        },
        [setState]
    );

    useEffect(() => {
        setState(serverside);
    }, [setState, serverside]);

    return [state, persistState];
}
