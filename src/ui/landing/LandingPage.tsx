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

export function LandingPage() {
    const { t } = useTranslation();
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
                        onClick={() =>
                            open({
                                canCreateDirectories: true,
                                directory: true,
                                recursive: true,
                                title: t("dialogs.createProject"),
                            }).then(console.log)
                        }
                    >
                        {t("landing.new.button")}
                    </Button>
                    <Button
                        leftSection={<TbFolderOpen size={20} />}
                        justify="space-between"
                        onClick={() =>
                            open({
                                canCreateDirectories: true,
                                directory: true,
                                recursive: true,
                                title: t("dialogs.openProject"),
                            }).then(console.log)
                        }
                    >
                        {t("landing.open.button")}
                    </Button>
                    <Divider opacity={0.2} />
                    <Button
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
