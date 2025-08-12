import { createContext } from "react";
import { IconTree } from "react-icons";

export type IconsContextType = {
    icons: { [key: string]: IconTree | null };
    request: (icon: string) => string;
    unrequest: (id: string) => void;
};

export const IconsContext = createContext<IconsContextType>(null as any);
