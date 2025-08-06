import {
    ActionIcon,
    ActionIconGroup,
    AppShell,
    Box,
    Button,
    Divider,
    Group,
    Menu,
    Stack,
    Tabs,
    Text,
} from "@mantine/core";
import "./style.scss";
import {
    TbFolder,
    TbFolderOpen,
    TbListTree,
    TbLogout2,
    TbMaximize,
    TbMinimize,
    TbNotebook,
    TbPackage,
    TbPlus,
    TbSettingsFilled,
    TbTagsFilled,
    TbTemplate,
    TbTools,
    TbX,
} from "react-icons/tb";
import { getCurrentWindow } from "@tauri-apps/api/window";
import AppIcon from "@/assets/icon.svg?react";
import { useTranslation } from "react-i18next";
import { Split } from "@gfazioli/mantine-split-pane";
import { Outlet, useNavigate } from "react-router";
import { useModals } from "@/modals";
import { open } from "@tauri-apps/plugin-dialog";
import api, { SerializableError } from "@/api";
import { useNotifications } from "@/notifications";

export function LayoutView() {
    const { t } = useTranslation();
    const { createProject } = useModals();
    const nav = useNavigate();
    const { error } = useNotifications();

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
                        <Menu shadow="sm" position="bottom-start">
                            <Menu.Target>
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
                            </Menu.Target>
                            <Menu.Dropdown>
                                <Menu.Item
                                    leftSection={<TbPlus size={16} />}
                                    onClick={() => {
                                        nav("/");
                                        createProject({});
                                    }}
                                >
                                    {t("menu.file.create")}
                                </Menu.Item>
                                <Menu.Item
                                    leftSection={<TbFolderOpen size={16} />}
                                    onClick={() => {
                                        nav("/");
                                        open({
                                            canCreateDirectories: false,
                                            directory: true,
                                            recursive: true,
                                            title: t("dialogs.openProject"),
                                            defaultPath: ".",
                                        }).then((path) => {
                                            if (path !== null) {
                                                api.application
                                                    .open_project(path)
                                                    .catch(
                                                        (
                                                            e: SerializableError
                                                        ) => {
                                                            error(
                                                                `Error code <${
                                                                    e.err
                                                                }>: ${
                                                                    e.err ===
                                                                    "no_active_project"
                                                                        ? "No project currently active."
                                                                        : e.context
                                                                }`
                                                            );
                                                        }
                                                    );
                                            }
                                        });
                                    }}
                                >
                                    {t("menu.file.open")}
                                </Menu.Item>
                                <Menu.Item
                                    leftSection={<TbLogout2 size={16} />}
                                    onClick={() =>
                                        api.application.exit_project()
                                    }
                                >
                                    {t("menu.file.exit")}
                                </Menu.Item>
                            </Menu.Dropdown>
                        </Menu>

                        <Menu shadow="sm" position="bottom-start">
                            <Menu.Target>
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
                            </Menu.Target>
                            <Menu.Dropdown>
                                <Menu.Item
                                    leftSection={<TbSettingsFilled size={16} />}
                                >
                                    {t("menu.project.settings")}
                                </Menu.Item>
                                <Menu.Divider />
                                <Menu.Label>
                                    {t("menu.project.resources")}
                                </Menu.Label>
                                <Menu.Item
                                    leftSection={<TbTemplate size={16} />}
                                >
                                    {t("menu.project.templates")}
                                </Menu.Item>
                                <Menu.Item
                                    leftSection={<TbTagsFilled size={16} />}
                                >
                                    {t("menu.project.categories")}
                                </Menu.Item>
                                <Menu.Item
                                    leftSection={<TbPackage size={16} />}
                                >
                                    {t("menu.project.packs")}
                                </Menu.Item>
                            </Menu.Dropdown>
                        </Menu>
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
                        <Stack gap={0} p={0} id="nav-pane"></Stack>
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
