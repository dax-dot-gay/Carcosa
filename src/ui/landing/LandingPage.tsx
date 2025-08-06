import {
    Button,
    Center,
    Divider,
    Group,
    Stack,
    Text,
    Title,
} from "@mantine/core";
import "./style.scss";
import AppIcon from "@/assets/icon.svg?react";
import { useTranslation } from "react-i18next";
import { TbFolderOpen, TbPlus, TbSettingsFilled } from "react-icons/tb";
import { open } from "@tauri-apps/plugin-dialog";
import { useModals } from "@/modals";
import api, { SerializableError } from "@/api";
import { useNavigate } from "react-router";
import { useDisclosure } from "@mantine/hooks";
import { useNotifications } from "@/notifications";

export function LandingPage() {
    const { t } = useTranslation();
    const { createProject } = useModals();
    const nav = useNavigate();
    const [loading, { open: startLoading, close: stopLoading }] =
        useDisclosure(false);
    const { error } = useNotifications();
    return (
        <Center id="landing-page">
            <Stack gap="xl" justify="center" align="center">
                <Group gap="md">
                    <AppIcon id="landing-page-icon" />
                    <Stack gap={0} justify="start" align="start">
                        <Title order={2} ff="monospace" fw={400}>
                            {t("app.name")}
                        </Title>
                        <Text size="sm" c="dimmed" ff="monospace">
                            {t("app.subtitle")}
                        </Text>
                    </Stack>
                </Group>
                <Stack id="landing-page-menu" gap="sm">
                    <Button
                        leftSection={<TbPlus size={20} />}
                        justify="space-between"
                        loading={loading}
                        onClick={() => createProject({})}
                    >
                        {t("landing.new.button")}
                    </Button>
                    <Button
                        loading={loading}
                        leftSection={<TbFolderOpen size={20} />}
                        justify="space-between"
                        onClick={() => {
                            startLoading();
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
                                        .then(() => {
                                            nav("/project");
                                            stopLoading();
                                        })
                                        .catch((e: SerializableError) => {
                                            stopLoading();
                                            error(
                                                `Error code <${e.err}>: ${
                                                    e.err ===
                                                    "no_active_project"
                                                        ? "No project currently active."
                                                        : e.context
                                                }`
                                            );
                                        });
                                } else {
                                    stopLoading();
                                }
                            });
                        }}
                    >
                        {t("landing.open.button")}
                    </Button>
                    <Divider opacity={0.2} />
                    <Button
                        loading={loading}
                        leftSection={<TbSettingsFilled size={20} />}
                        justify="space-between"
                        variant="light"
                    >
                        {t("landing.settings.button")}
                    </Button>
                </Stack>
            </Stack>
        </Center>
    );
}
