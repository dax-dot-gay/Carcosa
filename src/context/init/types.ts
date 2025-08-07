import { State } from "@/api";
import { createContext } from "react";

export type InitContextType = {
    state: State;
};

export const InitContext = createContext<InitContextType>(null as any);
