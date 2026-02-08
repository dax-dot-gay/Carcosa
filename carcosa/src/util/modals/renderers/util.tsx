import { ContextModalProps, modals } from "@mantine/modals";
import { ModalOptions } from "../types";
import { randomId } from "@mantine/hooks";
import { Group, Title } from "@mantine/core";

export function generateModal<P extends object = {}>(
    options: ModalOptions<P>,
): {
    id: string;
    renderer: (props: ContextModalProps<P>) => void;
    opener: (props: P) => string;
} {
    const ModalIcon = options.icon;
    return {
        id: options.id,
        renderer({ context, id, innerProps }) {
            const Renderer = options.renderer;
            return <Renderer context={context} id={id} {...innerProps} />;
        },
        opener(props) {
            return modals.openContextModal({
                modalId: randomId(options.id + "-"),
                modal: options.id,
                className: `context-modal modal-${options.id}`,
                title: ModalIcon ? (
                    <Group gap="sm" wrap="nowrap">
                        <ModalIcon size={24} />
                        <Title order={3} fw={500} ff="monospace">
                            {options.title}
                        </Title>
                    </Group>
                ) : (
                    <Title order={3} fw={500} ff="monospace">
                        {options.title}
                    </Title>
                ),
                innerProps: props,
                ...(options.settings ?? {}),
            });
        },
    };
}
