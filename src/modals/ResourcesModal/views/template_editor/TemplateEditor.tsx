import api, { SerializableError, Template, writeError } from "@/api";
import { IconSelector } from "@/components/DynamicIcons";
import { useParams } from "@/context/routing";
import {
    Alert,
    Center,
    Divider,
    Group,
    Loader,
    Stack,
    Textarea,
    TextInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { TbPencil, TbX } from "react-icons/tb";

type TemplateEditorParams = {
    mode: "edit" | "view";
    id: string;
};

function TemplateEditorInterface({
    mode,
    template,
}: {
    mode: string;
    template: Template;
}) {
    const { t } = useTranslation();

    const formState = useForm<{
        icon: string | null;
        name: string;
        description: string;
    }>({
        initialValues: {
            icon: template.icon ?? null,
            name: template.name,
            description: template.description ?? "",
        },
    });
    return (
        <form
            onSubmit={formState.onSubmit((values) => {
                console.log(values);
            })}
        >
            <Stack
                gap="sm"
                p={0}
                className="rm-view template-editor"
                data-mode={mode}
            >
                <Group p="sm" gap="sm" wrap="nowrap" pb={0}>
                    <IconSelector
                        size={42}
                        variant="light"
                        iconSize={24}
                        {...formState.getInputProps("icon")}
                    />
                    <TextInput
                        {...formState.getInputProps("name")}
                        variant="filled"
                        placeholder={t("modals.resources.views.templates.name")}
                        size="md"
                        leftSection={<TbPencil size={20} />}
                        style={{ flexGrow: 1 }}
                    />
                </Group>
                <Textarea
                    {...formState.getInputProps("description")}
                    w="calc(100% - 2 * var(--mantine-spacing-sm))"
                    rows={2}
                    placeholder={t(
                        "modals.resources.views.templates.description",
                    )}
                    mx="sm"
                    variant="filled"
                />
                <Divider />
            </Stack>
        </form>
    );
}

type LoadResult =
    | {
          kind: "loading";
      }
    | {
          kind: "error";
          error: SerializableError;
      }
    | {
          kind: "loaded";
          result: Template;
      }
    | {
          kind: "not_found";
      };

export function TemplateEditor() {
    const { mode, id } = useParams() as TemplateEditorParams;
    const { t } = useTranslation();
    const [active, setActive] = useState<LoadResult>({ kind: "loading" });

    useEffect(() => {
        api.templates
            .get_template_by_uuid(id)
            .then((result) => {
                if (result) {
                    setActive({ kind: "loaded", result });
                } else {
                    setActive({ kind: "not_found" });
                }
            })
            .catch((e) => setActive({ kind: "error", error: e }));
    }, [id, setActive]);

    switch (active.kind) {
        case "loading":
            return (
                <Center h="100%" w="100%">
                    <Loader />
                </Center>
            );
        case "error":
            return (
                <Alert
                    color="red"
                    icon={<TbX />}
                    title={t("common.notif.error")}
                >
                    {t("errors.unknown", writeError(active.error, t))}
                </Alert>
            );
        case "not_found":
            return (
                <Alert
                    color="red"
                    icon={<TbX />}
                    title={t("common.notif.error")}
                >
                    {t("errors.notFound")}
                </Alert>
            );
        case "loaded":
            return (
                <TemplateEditorInterface mode={mode} template={active.result} />
            );
    }
}
