import api from "@/api";
import { IconsProvider } from "@/components/DynamicIcons";
import { useEvent } from "@/events";
import { shadcnCssVariableResolver } from "@/theme/cssVariableResolver";
import { shadcnTheme } from "@/theme/theme";
import { MantineProvider } from "@mantine/core";
import { ModalsProvider } from "@mantine/modals";
import { Notifications } from "@mantine/notifications";
import { useEffect } from "react";
import { Outlet, useLocation, useNavigate } from "react-router";

export function Wrapper() {
    const nav = useNavigate();
    const location = useLocation();
    useEffect(() => {
        if (location.pathname === "/") {
            api.application.project_config().then((_) => nav("/project"));
        }
    }, [nav, location.pathname]);

    useEvent(api.application.opened_project, () => nav("/project"), [nav]);
    useEvent(api.application.closed_project, () => nav("/"), [nav]);

    return (
        <IconsProvider>
            <MantineProvider
                theme={shadcnTheme}
                cssVariablesResolver={shadcnCssVariableResolver}
                forceColorScheme="dark"
            >
                <ModalsProvider>
                    <Notifications />
                    <Outlet />
                </ModalsProvider>
            </MantineProvider>
        </IconsProvider>
    );
}
