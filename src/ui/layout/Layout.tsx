import {
    ActionIcon,
    ActionIconGroup,
    AppShell,
    Box,
    Button,
    Divider,
    Group,
} from "@mantine/core";
import "./style.scss";
import {
    TbFolder,
    TbMaximize,
    TbMinimize,
    TbNotebook,
    TbTools,
    TbX,
} from "react-icons/tb";
import { getCurrentWindow } from "@tauri-apps/api/window";
import AppIcon from "@/assets/icon.svg?react";
import { useTranslation } from "react-i18next";
import { Split } from "@gfazioli/mantine-split-pane";
import { Outlet } from "react-router";
export function LayoutView() {
    const { t } = useTranslation();

    return (
        <AppShell
            id="app"
            header={{
                height: 48,
            }}
        >
            <AppShell.Header id="app-header" pr={6}>
                <Group gap="sm" justify="space-between" id="header-root">
                    <Group gap={0} p={0} id="header-content">
                        <ActionIcon
                            radius={0}
                            className="header-button"
                            id="app-icon-button"
                            variant="subtle"
                        >
                            <AppIcon id="app-icon" />
                        </ActionIcon>
                        <Divider
                            orientation="vertical"
                            className="header-divider"
                        />
                        <Button
                            radius={0}
                            className="header-button"
                            leftSection={<TbFolder size={18} />}
                            variant="subtle"
                            color="gray"
                            size="sm"
                        >
                            {t("menu.file.button")}
                        </Button>
                        <Button
                            radius={0}
                            className="header-button"
                            leftSection={<TbNotebook size={18} />}
                            variant="subtle"
                            color="gray"
                            size="sm"
                        >
                            {t("menu.project.button")}
                        </Button>
                        <Button
                            radius={0}
                            className="header-button"
                            leftSection={<TbTools size={18} />}
                            variant="subtle"
                            color="gray"
                            size="sm"
                        >
                            {t("menu.tools.button")}
                        </Button>
                    </Group>
                    <Group gap="sm" id="header-controls">
                        <ActionIconGroup>
                            <ActionIcon
                                variant="subtle"
                                size="lg"
                                aria-label="Minimize"
                                color="gray"
                                onClick={() => getCurrentWindow().minimize()}
                            >
                                <TbMinimize size={20} />
                            </ActionIcon>
                            <ActionIcon
                                variant="subtle"
                                size="lg"
                                aria-label="Maximize"
                                color="gray"
                                onClick={() => getCurrentWindow().maximize()}
                            >
                                <TbMaximize size={20} />
                            </ActionIcon>
                            <ActionIcon
                                variant="subtle"
                                size="lg"
                                aria-label="Close"
                                color="red"
                                onClick={() => getCurrentWindow().close()}
                            >
                                <TbX size={20} />
                            </ActionIcon>
                        </ActionIconGroup>
                    </Group>
                </Group>
            </AppShell.Header>
            <AppShell.Main id="app-main">
                <Split className="app-split">
                    <Split.Pane minWidth={150} maxWidth={600}>
                        <Box id="nav-pane" p="sm"></Box>
                    </Split.Pane>
                    <Split.Resizer
                        className="app-split-handle"
                        variant="transparent"
                        opacity={0}
                    />
                    <Split.Pane grow>
                        <Box id="content-pane" p="sm">
                            <Outlet />
                        </Box>
                    </Split.Pane>
                </Split>
            </AppShell.Main>
        </AppShell>
    );
}
