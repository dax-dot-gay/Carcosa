import { IconSelector } from "@/components/DynamicIcons";
import { useParams } from "@/context/routing";
import { Group, Stack } from "@mantine/core";
import { useTranslation } from "react-i18next";

type TemplateEditorParams =
    | {
          mode: "create";
          id: undefined;
      }
    | {
          mode: "edit" | "view";
          id: string;
      };

export function TemplateEditor() {
    const { t } = useTranslation();
    const { mode, id } = useParams() as TemplateEditorParams;
    return (
        <Stack
            gap="sm"
            p={0}
            className="rm-view template-editor"
            data-mode={mode}
        >
            <Group p="sm">
                <IconSelector size="lg" variant="light" iconSize={20} />
            </Group>
        </Stack>
    );
}
