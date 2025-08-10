import { usePersistedState } from "@/context/init";
import { Outlet, useNavigate } from "@/context/routing";
import { Split } from "@gfazioli/mantine-split-pane";
import {
    Stack,
    Box,
    ScrollArea,
    Button,
    Collapse,
    Divider,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { IconType } from "react-icons";
import {
    TbChevronDown,
    TbPackage,
    TbPackageImport,
    TbPlus,
    TbSettings2,
    TbTemplate,
} from "react-icons/tb";

function Expander({
    label,
    icon,
    open,
    onToggle,
    children,
}: {
    label: string;
    icon: IconType;
    open: boolean;
    onToggle: () => void;
    children?: ReactNode | ReactNode[];
}) {
    let Icon = icon;
    return (
        <Box className="menu-expander">
            <Button
                variant={open ? "filled" : "outline"}
                onClick={onToggle}
                leftSection={<Icon size={18} />}
                className="menu-expander-button"
                rightSection={
                    <TbChevronDown
                        className="expander-chevron"
                        size={18}
                        style={{
                            transform: open ? "rotate(180deg)" : "rotate(0deg)",
                        }}
                    />
                }
                fullWidth
                size="xs"
                justify="start"
            >
                {label}
            </Button>
            <Collapse
                in={open}
                className="menu-expander-content"
                py="xs"
                pl="xs"
            >
                {children}
            </Collapse>
        </Box>
    );
}

export function ModalLayout() {
    const { t } = useTranslation();
    const [sidebarWidth, setSidebarWidth] = usePersistedState(
        "resource_manager_sidebar_width",
    );

    const [docTemplates, { toggle: toggleDocTemplates }] = useDisclosure(true);
    const [coreTemplates, { toggle: toggleCoreTemplates }] =
        useDisclosure(false);
    const [packages, { toggle: togglePackages }] = useDisclosure(false);
    const nav = useNavigate();

    return (
        <Split
            className="resource-manager"
            variant="transparent"
            hoverColor="dark.7"
            spacing="sm"
            size="sm"
            withKnob
            knobAlwaysOn
            knobColor="dark.7"
            knobHoverColor="dark.6"
        >
            <Split.Pane
                minWidth={150}
                maxWidth={400}
                initialWidth={sidebarWidth}
                onResizeEnd={({ width }) =>
                    setSidebarWidth(Number.parseInt(width.toFixed(0)))
                }
            >
                <Box id="rm-nav" p={0}>
                    <ScrollArea id="rm-nav-scroll">
                        <Stack gap={"xs"} p="xs">
                            <Expander
                                label={t(
                                    "modals.resources.nav.docTemplates.title",
                                )}
                                icon={TbTemplate}
                                open={docTemplates}
                                onToggle={toggleDocTemplates}
                            >
                                <Stack gap="xs" p={0}>
                                    <Divider />
                                    <Button
                                        size="xs"
                                        variant="light"
                                        fullWidth
                                        leftSection={<TbPlus size={18} />}
                                        justify="space-between"
                                        onClick={() => nav("/templates/create")}
                                    >
                                        {t("actions.create")}
                                    </Button>
                                </Stack>
                            </Expander>
                            <Expander
                                label={t(
                                    "modals.resources.nav.coreTemplates.title",
                                )}
                                icon={TbSettings2}
                                open={coreTemplates}
                                onToggle={toggleCoreTemplates}
                            >
                                <Button size="xs" variant="light" fullWidth>
                                    EEE
                                </Button>
                            </Expander>
                            <Expander
                                label={t("modals.resources.nav.packages.title")}
                                icon={TbPackage}
                                open={packages}
                                onToggle={togglePackages}
                            >
                                <Stack gap="xs" p={0}>
                                    <Divider />
                                    <Button
                                        size="xs"
                                        variant="light"
                                        fullWidth
                                        leftSection={
                                            <TbPackageImport size={18} />
                                        }
                                        justify="space-between"
                                    >
                                        {t("actions.import")}
                                    </Button>
                                </Stack>
                            </Expander>
                        </Stack>
                    </ScrollArea>
                </Box>
            </Split.Pane>
            <Split.Resizer className="rm-split-handle" />
            <Split.Pane grow>
                <Box id="rm-content" p="sm">
                    <Outlet />
                </Box>
            </Split.Pane>
        </Split>
    );
}
