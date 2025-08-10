import { useNavigate } from "@/context/routing";
import { Button, Center, Group, Stack, Text } from "@mantine/core";
import { useTranslation } from "react-i18next";
import { TbCategoryFilled, TbPackage, TbTemplate } from "react-icons/tb";

export function IndexView() {
    const { t } = useTranslation();
    const nav = useNavigate();
    return (
        <Center className="rm-view index" h="100%">
            <Stack gap="xl" justify="center">
                <Group
                    gap="md"
                    justify="start"
                    opacity={0.3}
                    style={{ pointerEvents: "none", WebkitUserSelect: "none" }}
                >
                    <TbCategoryFilled size="6em" />
                    <Stack gap={4} justify="center" align="start">
                        <Text size="3em" ff="monospace" lh="42px">
                            {t("modals.resources.views.index.resource")}
                        </Text>
                        <Text size="2em" ff="monospace" lh="36px">
                            {t("modals.resources.views.index.manager")}
                        </Text>
                    </Stack>
                </Group>
                <Group gap="sm">
                    <Button
                        size="md"
                        leftSection={<TbTemplate size={24} />}
                        variant="light"
                        onClick={() => nav("/templates/create")}
                    >
                        {t("modals.resources.views.index.createTemplate")}
                    </Button>
                    <Button
                        size="md"
                        leftSection={<TbPackage size={24} />}
                        variant="light"
                    >
                        {t("modals.resources.views.index.addPackage")}
                    </Button>
                </Group>
            </Stack>
        </Center>
    );
}
