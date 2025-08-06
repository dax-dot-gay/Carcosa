import * as bindings from "../bindings";

export * from "../bindings";
export type * from "../bindings";

const api = bindings.createTauRPCProxy();

export default api;
