import api from "@/api";
import { useDebouncedValue, useSet } from "@mantine/hooks";
import { ReactNode, useEffect, useState } from "react";
import { IconTree } from "react-icons";
import { IconsContext } from "./util";

export function IconsProvider({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    const requests = useSet<string>([]);
    const [icons, setIcons] = useState<{ [key: string]: IconTree | null }>({});
    const [debouncedRequests] = useDebouncedValue(requests, 750, {
        leading: true,
    });

    const checked = JSON.stringify(Array.from(requests.values()));

    useEffect(() => {
        if (debouncedRequests.size > 0) {
            api.application.icons
                .icons(Array.from(debouncedRequests))
                .then((results) => {
                    setIcons(
                        Object.fromEntries(
                            Object.entries(results).map(([k, v]) => [
                                k,
                                v ? JSON.parse(v) : null,
                            ]),
                        ),
                    );
                });
        }
    }, [checked, setIcons]);

    return (
        <IconsContext.Provider
            value={{
                icons,
                request: (icon) => {
                    requests.add(icon);
                },
                unrequest: (icon) => {
                    requests.delete(icon);
                },
            }}
        >
            {children}
        </IconsContext.Provider>
    );
}
