import { ModalsProvider } from "@mantine/modals";
import { ReactNode } from "react";
import renderers from "./renderers";

export function ApplicationModals({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    return (
        <ModalsProvider modals={renderers.renderers as any}>
            {children}
        </ModalsProvider>
    );
}
