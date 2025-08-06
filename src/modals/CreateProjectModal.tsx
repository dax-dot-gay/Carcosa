import { TbFolderPlus, TbPencil, TbPlus, TbX } from "react-icons/tb";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { createModalOpener } from "./common";
import { useTranslation } from "react-i18next";
import {
    Button,
    Divider,
    Group,
    LoadingOverlay,
    Stack,
    Textarea,
    TextInput,
} from "@mantine/core";
import { open as dialogOpen } from "@tauri-apps/plugin-dialog";
import { modals } from "@mantine/modals";
import api, { SerializableError } from "@/api";
import { useNavigate } from "react-router";
import { useNotifications } from "@/notifications";

const openCreateProjectModal = createModalOpener({
    id: "create_project",
    title: "modals.createProject.title",
    icon: TbPlus,
    renderer(_) {
        const { t } = useTranslation();
        const nav = useNavigate();
        const [loading, { open: startLoading, close: stopLoading }] =
            useDisclosure(false);
        const form = useForm({
            initialValues: {
                name: "",
                path: "",
                description: "",
            },
            validate: {
                name: (v) => (v.length > 0 ? null : t("errors.form.empty")),
                path: (v) => (v.length > 0 ? null : t("errors.form.empty")),
            },
        });
        const { success, error } = useNotifications();

        return (
            <form
                onSubmit={form.onSubmit((values) => {
                    startLoading();
                    api.application
                        .create_project({
                            path: values.path,
                            name: values.name,
                            description:
                                values.description.length > 0
                                    ? values.description
                                    : null,
                        })
                        .then((_) => {
                            success(t("modals.createProject.success"));
                            modals.closeAll();
                            nav("/project");
                            stopLoading();
                        })
                        .catch((e: SerializableError) => {
                            stopLoading();
                            error(
                                `Error code <${e.err}>: ${
                                    e.err === "no_active_project"
                                        ? "No project currently active."
                                        : e.context
                                }`
                            );
                        });
                })}
            >
                <LoadingOverlay visible={loading} />
                <Stack gap="sm">
                    <TextInput
                        className="modal-input create-path"
                        leftSection={<TbFolderPlus size={20} />}
                        withAsterisk
                        onClick={() =>
                            dialogOpen({
                                directory: true,
                                title: t("dialogs.createProject"),
                                canCreateDirectories: true,
                                recursive: true,
                                defaultPath: ".",
                            }).then((value) =>
                                form.setFieldValue(
                                    "path",
                                    value === null ? "" : value
                                )
                            )
                        }
                        label={t("modals.createProject.path.label")}
                        placeholder={t("modals.createProject.path.placeholder")}
                        {...form.getInputProps("path")}
                    />
                    <Divider />
                    <TextInput
                        className="modal-input"
                        withAsterisk
                        leftSection={<TbPencil size={20} />}
                        label={t("modals.createProject.name.label")}
                        placeholder="The Library"
                        {...form.getInputProps("name")}
                    />
                    <Textarea
                        className="modal-input"
                        rows={4}
                        label={t("modals.createProject.description.label")}
                        placeholder="Song of my soul, my voice is dead, / Die thou, unsung, as tears unshed / Shall dry and die in / Lost Carcosa."
                        {...form.getInputProps("description")}
                    />
                    <Group gap="sm" justify="space-between">
                        <Button
                            leftSection={<TbX size={20} />}
                            variant="light"
                            color="gray"
                            onClick={() => modals.closeAll()}
                        >
                            {t("actions.cancel")}
                        </Button>
                        <Button
                            leftSection={<TbPlus size={20} />}
                            type="submit"
                        >
                            {t("actions.create")}
                        </Button>
                    </Group>
                </Stack>
            </form>
        );
    },
    size: "lg",
    centered: true,
});

export default openCreateProjectModal;
