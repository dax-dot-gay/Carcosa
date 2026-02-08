import {
    ActionIcon,
    Button,
    Center,
    Divider,
    Group,
    Stack,
    TextInput,
    Title,
    Tooltip,
} from "@mantine/core";
import {
    TbBook2,
    TbCircleXFilled,
    TbFolderOpen,
    TbLink,
    TbPlugConnected,
    TbPlus,
} from "react-icons/tb";
import { useInputState } from "@mantine/hooks";
import { openModal } from "../../util/modals";
import { useIpc } from "../../util/api";
import { notifications } from "@mantine/notifications";
import { modals } from "@mantine/modals";

export function StartupView() {
    const [remoteAddr, setRemoteAddr] = useInputState("");
    const ipc = useIpc();
    return (
        <Center w="100vw" h="100vh">
            <Stack gap="lg" align="center" w="min(calc(100% - 64px), 384px)">
                <Group gap="sm" w="100%" justify="space-between">
                    <TbBook2 size={36} />
                    <Title order={1} fw={400} ff="monospace">
                        Carcosa
                    </Title>
                </Group>
                <Divider w="100%" />
                <Stack gap="sm" align="center" w="100%">
                    <Button
                        fullWidth
                        leftSection={<TbPlus size={24} />}
                        size="lg"
                        justify="space-between"
                        onClick={() =>
                            openModal("create_project", {
                                onSubmit(id, name, path) {
                                    ipc.projects
                                        .create_project(name, path)
                                        .resolve((result) => {
                                            if (result.success) {
                                                modals.close(id);
                                            } else {
                                                console.log(
                                                    `ipc_error: ${result.error} - "${result.message}"`,
                                                );
                                                notifications.show({
                                                    color: "red",
                                                    title: "Failed to create project!",
                                                    message: result.message,
                                                    icon: <TbCircleXFilled />,
                                                });
                                            }
                                        });
                                },
                            })
                        }
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
                    <Group gap="sm" wrap="nowrap" w="100%">
                        <TextInput
                            size="md"
                            leftSection={<TbLink size={20} />}
                            placeholder="Remote Host Address"
                            style={{ flexGrow: 1 }}
                            value={remoteAddr}
                            onChange={setRemoteAddr}
                        />
                        <Tooltip
                            label="Connect to a remote host"
                            position="bottom-end"
                        >
                            <ActionIcon
                                size="40"
                                disabled={remoteAddr.length === 0}
                            >
                                <TbPlugConnected size={24} />
                            </ActionIcon>
                        </Tooltip>
                    </Group>
                </Stack>
            </Stack>
        </Center>
    );
}
