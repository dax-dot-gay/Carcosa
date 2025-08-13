import api, { LayoutKind, SerializableError } from "@/api";
import { IconSelector } from "@/components/DynamicIcons";
import { useNavigate } from "@/context/routing";
import { useNotifications } from "@/notifications";
import {
    Stack,
    Group,
    TextInput,
    Textarea,
    Divider,
    Center,
    Paper,
    Text,
    Button,
    useCombobox,
    Combobox,
    InputBase,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { IconType } from "react-icons";
import {
    TbCalendar,
    TbFileText,
    TbForms,
    TbLayout,
    TbMap2,
    TbPencil,
    TbPlus,
    TbSelector,
    TbTemplate,
    TbTimelineEventExclamation,
    TbX,
} from "react-icons/tb";

const layoutIcons: { [key in LayoutKind]: IconType } = {
    form: TbForms,
    rich_document: TbFileText,
    interactable_map: TbMap2,
    calendar: TbCalendar,
    timeline: TbTimelineEventExclamation,
};

function LayoutOption({ mode }: { mode: LayoutKind }) {
    const Icon = layoutIcons[mode];
    const { t } = useTranslation();
    return (
        <Stack gap={0}>
            <Group gap={6} wrap="nowrap">
                <Icon size={18} />
                <span>
                    {t(`modals.resources.views.templates.layouts.${mode}`)}
                </span>
            </Group>
            <Text c="dimmed" size="xs">
                {t(`modals.resources.views.templates.layout_descs.${mode}`)}
            </Text>
        </Stack>
    );
}

function LayoutSelector({
    value,
    onChange,
}: {
    value: LayoutKind;
    onChange: (layout: LayoutKind) => void;
}) {
    const { t } = useTranslation();
    const combobox = useCombobox({
        onDropdownClose: () => combobox.resetSelectedOption(),
    });
    const options = Object.keys(layoutIcons).map((k) => (
        <Combobox.Option value={k} key={k}>
            <LayoutOption mode={k as any} />
        </Combobox.Option>
    ));
    return (
        <Stack gap={6}>
            <Group gap="xs" w="100%">
                <TbLayout size={18} />
                <Text size="sm">
                    {t("modals.resources.views.templates.layoutLabel")}
                </Text>
                <Divider style={{ flexGrow: 1 }} variant="dashed" />
            </Group>
            <Combobox
                store={combobox}
                withinPortal={false}
                onOptionSubmit={(val) => {
                    onChange(val as any);
                    combobox.closeDropdown();
                }}
            >
                <Combobox.Target>
                    <InputBase
                        component="button"
                        type="button"
                        pointer
                        rightSection={<TbSelector size={20} />}
                        onClick={() => combobox.toggleDropdown()}
                        rightSectionPointerEvents="none"
                        variant="filled"
                        multiline
                        size="md"
                    >
                        <LayoutOption mode={value} />
                    </InputBase>
                </Combobox.Target>
                <Combobox.Dropdown>
                    <Combobox.Options>{options}</Combobox.Options>
                </Combobox.Dropdown>
            </Combobox>
        </Stack>
    );
}

export function TemplateCreator() {
    const { t } = useTranslation();
    const form = useForm<{
        icon: string | null;
        name: string;
        description: string;
        layout: LayoutKind;
    }>({
        initialValues: {
            icon: "tb_template",
            name: "",
            description: "",
            layout: "form",
        },
    });
    const nav = useNavigate();
    const [loading, setLoading] = useState(false);
    const { fromError } = useNotifications();

    return (
        <Center w="100%" h="100%">
            <form
                onSubmit={form.onSubmit((values) => {
                    setLoading(true);
                    api.templates
                        .create_template(values)
                        .then((created) => {
                            setLoading(false);
                            nav(`/templates/edit/${created.id}`);
                        })
                        .catch((reason: SerializableError) => {
                            fromError(reason);
                            setLoading(false);
                        });
                })}
                style={{
                    width: "90%",
                    maxWidth: "512px",
                }}
            >
                <Stack gap="sm" justify="center" align="center">
                    <Group gap="sm" w="100%">
                        <TbTemplate size={28} />
                        <Text size="lg">
                            {t("modals.resources.views.templates.create.title")}
                        </Text>
                        <Divider style={{ flexGrow: 1 }} variant="dashed" />
                    </Group>
                    <Paper p="sm" withBorder shadow="xs" w="100%">
                        <Stack gap="sm">
                            <Group gap="sm" wrap="nowrap">
                                <IconSelector
                                    size={42}
                                    variant="light"
                                    iconSize={24}
                                    {...form.getInputProps("icon")}
                                />
                                <TextInput
                                    {...form.getInputProps("name")}
                                    variant="filled"
                                    placeholder={t(
                                        "modals.resources.views.templates.name",
                                    )}
                                    size="md"
                                    leftSection={<TbPencil size={20} />}
                                    style={{ flexGrow: 1 }}
                                    required
                                />
                            </Group>
                            <Textarea
                                {...form.getInputProps("description")}
                                w="100%"
                                rows={6}
                                placeholder={t(
                                    "modals.resources.views.templates.description",
                                )}
                                variant="filled"
                            />
                            <LayoutSelector
                                value={form.values.layout}
                                onChange={(value) =>
                                    form.setValues({ layout: value })
                                }
                            />
                        </Stack>
                    </Paper>
                    <Group gap="sm" grow wrap="nowrap" w="100%">
                        <Button
                            variant="light"
                            color="red"
                            leftSection={<TbX size={20} />}
                            justify="space-between"
                            onClick={() => nav("/")}
                            loading={loading}
                        >
                            {t("actions.cancel")}
                        </Button>
                        <Button
                            leftSection={<TbPlus size={20} />}
                            justify="space-between"
                            type="submit"
                            loading={loading}
                        >
                            {t("actions.create")}
                        </Button>
                    </Group>
                </Stack>
            </form>
        </Center>
    );
}
