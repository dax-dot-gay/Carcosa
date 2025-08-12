import api from "@/api";
import { randomId, useDebouncedValue, useMap, useSet } from "@mantine/hooks";
import { ReactNode, useEffect, useState } from "react";
import { IconTree } from "react-icons";
import { IconsContext } from "./util";
import { uniq } from "lodash";

export function IconsProvider({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    const requests = useMap<string, string>([]);
    const [icons, setIcons] = useState<{ [key: string]: IconTree | null }>({});
    const [debouncedRequests] = useDebouncedValue(requests, 750, {
        leading: true,
    });

    const checked = JSON.stringify(Array.from(requests.values()));

    useEffect(() => {
        if (debouncedRequests.size > 0) {
            api.application.icons
                .icons(uniq(Array.from(debouncedRequests.values())))
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
                    const id = randomId();
                    requests.set(id, icon);
                    return id;
                },
                unrequest: (id) => {
                    requests.delete(id);
                },
            }}
        >
            {children}
        </IconsContext.Provider>
    );
}
