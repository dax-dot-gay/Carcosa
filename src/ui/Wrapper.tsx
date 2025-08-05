import { shadcnCssVariableResolver } from "@/theme/cssVariableResolver";
import { shadcnTheme } from "@/theme/theme";
import { MantineProvider } from "@mantine/core";
import { ModalsProvider } from "@mantine/modals";
import { Notifications } from "@mantine/notifications";
import { Outlet } from "react-router";

export function Wrapper() {
    return (
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
    );
}
