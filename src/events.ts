import { UnlistenFn } from "@tauri-apps/api/event";
import { useEffect } from "react";

export function useEvent<L extends Function>(
    event: {
        on: (listener: L) => Promise<UnlistenFn>;
    },
    callback: L,
    deps?: any[]
) {
    useEffect(() => {
        let unlisten = event.on(callback);
        return () => {
            unlisten.then((f) => f());
        };
    }, deps ?? []);
}
