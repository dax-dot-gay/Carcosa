import { Button, Center, Divider, Group, Stack, Title } from "@mantine/core";
import { TbBook2, TbFolderOpen, TbPlus } from "react-icons/tb";

export function StartupView() {
    return (
        <Center w="100vw" h="100vh">
            <Stack gap="lg" align="center" w="min(calc(100% - 64px), 256px)">
                <Group gap="sm" w="100%">
                    <TbBook2 size={36} />
                    <Divider style={{ flexGrow: 1 }} />
                    <Title order={1} fw={400} ff="monospace">
                        Carcosa
                    </Title>
                </Group>
                <Stack gap="sm" align="center" w="100%">
                    <Button
                        fullWidth
                        leftSection={<TbPlus size={24} />}
                        size="lg"
                        justify="space-between"
                    >
                        New Project
                    </Button>
                    <Button
                        fullWidth
                        leftSection={<TbFolderOpen size={24} />}
                        size="lg"
                        justify="space-between"
                    >
                        Open Project
                    </Button>
                </Stack>
            </Stack>
        </Center>
    );
}
