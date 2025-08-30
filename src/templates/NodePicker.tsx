import { Button, ButtonProps, Group, Menu } from "@mantine/core";
import { useTranslation } from "react-i18next";
import { TbPlus } from "react-icons/tb";
import {
    NODE_CATEGORIES,
    NODE_CATEGORY_ICONS,
    NODE_KIND_ICONS,
    NODE_TYPES,
} from "./nodes";
import { IconType } from "react-icons";

export function NodePicker({
    onPick,
    ...props
}: { onPick: (category: string, kind: string) => void } & Partial<
    Omit<ButtonProps, "children" | "leftSection" | "rightSection" | "onClick">
>) {
    const { t } = useTranslation();
    return (
        <Menu position="bottom-start">
            <Menu.Target>
                <Button
                    {...props}
                    leftSection={
                        <TbPlus size="calc(var(--mantine-line-height) - 4px)" />
                    }
                >
                    {t("templates.edit.addNode")}
                </Button>
            </Menu.Target>
            <Menu.Dropdown>
                {NODE_CATEGORIES.map((category) => {
                    const CategoryIcon = NODE_CATEGORY_ICONS[category];
                    return (
                        <Menu.Sub key={category}>
                            <Menu.Sub.Target>
                                <Menu.Sub.Item
                                    leftSection={<CategoryIcon size={18} />}
                                >
                                    {t(`templates.nodes.${category}.self`)}
                                </Menu.Sub.Item>
                            </Menu.Sub.Target>
                            <Menu.Sub.Dropdown miw="192px">
                                <Menu.Label>
                                    <Group gap="sm" justify="space-between">
                                        <CategoryIcon size={16} />
                                        <span>
                                            {t(
                                                `templates.nodes.${category}.selfPlural`,
                                            )}
                                        </span>
                                    </Group>
                                </Menu.Label>
                                <Menu.Divider />
                                {NODE_TYPES[category].map((kind) => {
                                    const KindIcon = (NODE_KIND_ICONS as any)[
                                        category
                                    ][kind] as IconType;
                                    return (
                                        <Menu.Item
                                            key={`${category}.${kind}`}
                                            leftSection={<KindIcon size={18} />}
                                            onClick={() =>
                                                onPick(category, kind)
                                            }
                                        >
                                            {t(
                                                `templates.nodes.${category}.${kind}`,
                                            )}
                                        </Menu.Item>
                                    );
                                })}
                            </Menu.Sub.Dropdown>
                        </Menu.Sub>
                    );
                })}
            </Menu.Dropdown>
        </Menu>
    );
}
