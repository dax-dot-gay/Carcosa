import { createContext, useContext } from "react";
import { createTauRPCProxy } from "./bindings";

export type ApiType = ReturnType<typeof createTauRPCProxy>;
export const ApiContext = createContext<ApiType>(createTauRPCProxy());

export function useIpc(): ApiType {
    return useContext(ApiContext);
}

export type ApiResult<T> =
    | { success: true; result: T }
    | { success: false; error: string; message: string };

declare global {
    interface Promise<T> {
        resolve<R = void>(callback: (result: ApiResult<T>) => R): Promise<R>;
    }
}

Promise.prototype.resolve = function (callback) {
    return this.then((v) => callback({ success: true, result: v })).catch((v) =>
        callback({
            success: false,
            error: v.code,
            message: v.message,
        }),
    );
};
