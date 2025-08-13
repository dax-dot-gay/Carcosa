import api, {
    Node,
    SerializableError,
    Template,
    TemplateLayout,
    writeError,
} from "@/api";
import { IconSelector } from "@/components/DynamicIcons";
import { useParams } from "@/context/routing";
import {
    Alert,
    Box,
    Button,
    Center,
    Divider,
    Group,
    Loader,
    ScrollArea,
    Skeleton,
    Stack,
    Textarea,
    TextInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { TbCancel, TbDeviceFloppy, TbPencil, TbX } from "react-icons/tb";

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

    const state = useForm<{
        icon: string | null;
        name: string;
        description: string;
        layout: TemplateLayout;
        nodes: Partial<{ [key in string]: Node }>;
    }>({
        initialValues: {
            icon: template.icon ?? null,
            name: template.name,
            description: template.description ?? "",
            layout: template.layout,
            nodes: template.nodes ?? {},
        },
    });
    return (
        <Stack
            gap="sm"
            p={0}
            className="rm-view template-editor"
            data-mode={mode}
            h="100%"
            w="100%"
            style={{ overflow: "hidden" }}
        >
            <Group p="sm" gap="sm" wrap="nowrap" pb={0}>
                <IconSelector
                    size={42}
                    variant="light"
                    iconSize={24}
                    {...state.getInputProps("icon")}
                />
                <TextInput
                    {...state.getInputProps("name")}
                    variant="filled"
                    placeholder={t("modals.resources.views.templates.name")}
                    size="md"
                    leftSection={<TbPencil size={20} />}
                    style={{ flexGrow: 1 }}
                />
            </Group>
            <Textarea
                {...state.getInputProps("description")}
                w="calc(100% - 2 * var(--mantine-spacing-sm))"
                rows={2}
                placeholder={t("modals.resources.views.templates.description")}
                mx="sm"
                variant="filled"
            />
            <Box
                style={{
                    flexGrow: 1,
                    position: "relative",
                    overflow: "hidden",
                    borderTop: "1px solid var(--mantine-color-default-border)",
                    borderBottom:
                        "1px solid var(--mantine-color-default-border)",
                }}
                w="100%"
            >
                <ScrollArea
                    h="100%"
                    w="100%"
                    px="sm"
                    className="rm-view template-editor-scroll"
                >
                    <Box py="sm" className="rm-view template-editor-box"></Box>
                </ScrollArea>
            </Box>
            <Group gap="sm" justify="space-between" p="sm" wrap="nowrap" pt={0}>
                <Button
                    leftSection={<TbCancel size={20} />}
                    color="red"
                    variant="light"
                >
                    {t("actions.exit")}
                </Button>
                <Button leftSection={<TbDeviceFloppy size={20} />}>
                    {t("actions.save")}
                </Button>
            </Group>
        </Stack>
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
