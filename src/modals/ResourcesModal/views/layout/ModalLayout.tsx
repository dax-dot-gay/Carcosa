import { usePersistedState } from "@/context/init";
import { Outlet } from "@/context/routing";
import { Split } from "@gfazioli/mantine-split-pane";
import { Stack, Box } from "@mantine/core";
import { useTranslation } from "react-i18next";

export function ModalLayout() {
    const { t } = useTranslation();
    const [sidebarWidth, setSidebarWidth] = usePersistedState(
        "resource_manager_sidebar_width",
    );
    return (
        <Split
            className="resource-manager"
            variant="transparent"
            hoverColor="dark.7"
            spacing="sm"
            size="sm"
            withKnob
            knobAlwaysOn
            knobColor="dark.7"
            knobHoverColor="dark.6"
        >
            <Split.Pane
                minWidth={150}
                maxWidth={400}
                initialWidth={sidebarWidth}
                onResizeEnd={({ width }) =>
                    setSidebarWidth(Number.parseInt(width.toFixed(0)))
                }
            >
                <Stack gap={0} p={0} id="rm-nav"></Stack>
            </Split.Pane>
            <Split.Resizer className="rm-split-handle" />
            <Split.Pane grow>
                <Box id="rm-content" p="sm">
                    <Outlet />
                </Box>
            </Split.Pane>
        </Split>
    );
}
