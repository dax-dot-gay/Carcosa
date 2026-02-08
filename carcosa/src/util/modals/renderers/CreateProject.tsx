import { TbFolderFilled, TbHelp, TbPencil, TbPlus, TbX } from "react-icons/tb";
import { generateModal } from "./util";
import { Button, Group, Stack, TextInput, Tooltip } from "@mantine/core";
import { useForm } from "@mantine/form";
import { open as select_folder_dialog } from "@tauri-apps/plugin-dialog";
import { path } from "@tauri-apps/api";
import { ModalRendererProps } from "../types";

function CreateProjectModal({
    id,
    context,
    onSubmit,
}: ModalRendererProps<{
    onSubmit: (id: string, name: string, path: string) => void;
}>) {
    const createForm = useForm({
        initialValues: {
            name: "",
            path: "",
        },
        validate: {
            name: (value) =>
                value.length > 256
                    ? "Name is too long (max. 256)"
                    : value.includes("\\") || value.includes("/")
                      ? "Name contains illegal characters"
                      : null,
        },
    });
    return (
        <form
            onSubmit={createForm.onSubmit(({ name, path }) =>
                onSubmit(id, name, path),
            )}
        >
            <Stack gap="sm" w="100%">
                <TextInput
                    label="Project Name"
                    {...createForm.getInputProps("name")}
                    leftSection={<TbPencil size={20} />}
                    size="md"
                />
                <TextInput
                    label={
                        <Group gap={6}>
                            <span>Project Path</span>
                            <Tooltip
                                multiline
                                w={300}
                                withArrow
                                label="If the selected folder is empty and matches the project name, the selected folder will be used directly. Otherwise, a subfolder will be created with the project's name."
                            >
                                <TbHelp size={16} />
                            </Tooltip>
                        </Group>
                    }
                    className="path-input"
                    value={createForm.values.path}
                    readOnly
                    onClick={() =>
                        (async () => {
                            const root_path = await path.documentDir();
                            return await select_folder_dialog({
                                title: "Select Project Folder",
                                directory: true,
                                canCreateDirectories: true,
                                recursive: true,
                                defaultPath: root_path,
                            });
                        })().then((v) =>
                            createForm.setFieldValue("path", v ?? ""),
                        )
                    }
                    leftSection={<TbFolderFilled size={20} />}
                    size="md"
                />
                <Group gap="sm" justify="end">
                    <Button
                        justify="space-between"
                        variant="light"
                        color="red"
                        leftSection={<TbX size={20} />}
                        onClick={() => context.closeContextModal(id, true)}
                    >
                        Cancel
                    </Button>
                    <Button
                        justify="space-between"
                        leftSection={<TbPlus size={20} />}
                        type="submit"
                        disabled={
                            createForm.values.name === "" ||
                            createForm.values.path === ""
                        }
                    >
                        Create
                    </Button>
                </Group>
            </Stack>
        </form>
    );
}

export default generateModal({
    id: "create_project",
    title: "Create Project",
    icon: TbPlus,
    renderer: CreateProjectModal,
    settings: {
        centered: true,
    },
});
