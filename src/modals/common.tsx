import { Group, MantineSize, ModalProps, Text } from "@mantine/core";
import { modals } from "@mantine/modals";
import { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { IconType } from "react-icons";
import "./modals.scss";

export function ModalHeader({
    icon,
    title,
}: {
    icon: IconType;
    title: string;
}) {
    const { t } = useTranslation();
    let IconElement = icon;
    return (
        <Group gap="sm" justify="space-between" style={{ flexGrow: 2 }}>
            <IconElement size={24} />
            <Text>{t(title)}</Text>
        </Group>
    );
}

export function createModalOpener<Props extends Record<string, any> = {}>({
    id,
    title,
    icon,
    renderer,
    ...options
}: {
    id: string;
    title: string;
    icon: IconType;
    renderer: (props: Props) => ReactNode;
} & Partial<
    Omit<ModalProps, "opened" | "id" | "className" | "title" | "children">
>): (props: Props) => void {
    let Inner = renderer;

    return (props: Props) => {
        modals.open({
            id,
            title: <ModalHeader icon={icon} title={title} />,
            children: <Inner {...props} />,
            className: `modal modal-${id}`,
            ...options,
        });
    };
}
