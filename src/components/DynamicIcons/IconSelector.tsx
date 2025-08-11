import {
    ActionIcon,
    ActionIconProps,
    AspectRatio,
    Badge,
    Button,
    Center,
    Group,
    Modal,
    Paper,
    PaperProps,
    ScrollArea,
    SimpleGrid,
    Stack,
    Text,
    TextInput,
    Tooltip,
} from "@mantine/core";
import {
    useDebouncedValue,
    useInputState,
    useUncontrolled,
} from "@mantine/hooks";
import {
    createContext,
    forwardRef,
    useContext,
    useEffect,
    useMemo,
    useState,
} from "react";
import { IconType } from "react-icons";
import { TbPhoto, TbPhotoOff, TbSearch } from "react-icons/tb";
import { DynamicIcon } from "./DynamicIcon";
import { useTranslation } from "react-i18next";
import api from "@/api";
import Fuse from "fuse.js";
import { snakeCase, startCase } from "lodash";
import { VirtuosoGrid, VirtuosoGridProps } from "react-virtuoso";
import "./style.scss";

const gridComponents: VirtuosoGridProps<undefined, undefined>["components"] = {
    List: forwardRef(({ children, ...props }, ref) => (
        <SimpleGrid
            ref={ref}
            type="container"
            spacing={"xs"}
            cols={10}
            {...props}
        >
            {children}
        </SimpleGrid>
    )),
    Item: ({ children, ...props }) => <div {...props}>{children}</div>,
    Scroller: forwardRef((props, ref) => {
        // an alternative option to assign the ref is
        // <div ref={(r) => ref.current = r}>
        return <ScrollArea h="100%" w="100%" {...props} ref={ref} />;
    }),
};

const SelectionContext = createContext<{
    selected: string | null;
    setSelected: (name: string | null) => void;
}>(null as any);

function IconItem({
    name,
    ...props
}: {
    name: string;
} & Partial<PaperProps>) {
    const { t } = useTranslation();
    const { selected, setSelected } = useContext(SelectionContext);
    return (
        <AspectRatio ratio={1}>
            <Tooltip
                label={`${t(`icons.${name.split("_")[0]}`)} - ${startCase(name.split("_").slice(1).join(" "))}`}
                zIndex={2000}
                position="top"
                bg="var(--mantine-color-secondary-8)"
                c="var(--mantine-color-text)"
                withArrow
            >
                <Paper
                    bg="var(--mantine-color-secondary-8)"
                    p="xs"
                    radius="xs"
                    shadow="xs"
                    h="100%"
                    className={`icon-item ${selected === name ? "selected" : ""}`}
                    onClick={() => {
                        setSelected(name);
                    }}
                    {...props}
                >
                    <Badge size="xs" className="icon-item-badge" radius="xs">
                        {name.split("_")[0]}
                    </Badge>
                    <Center w="100%" h="100%">
                        <DynamicIcon icon={name} size={24} />
                    </Center>
                </Paper>
            </Tooltip>
        </AspectRatio>
    );
}

export function IconSelector({
    value,
    defaultValue,
    onChange,
    emptyIcon,
    iconSize,
    ...props
}: {
    value?: string | null;
    defaultValue?: string | null;
    onChange?: (value: string | null) => void;
    emptyIcon?: IconType;
    iconSize?: string | number;
} & Partial<Omit<ActionIconProps, "onClick" | "children">>) {
    const { t } = useTranslation();
    const [_value, handleChange] = useUncontrolled({
        value,
        defaultValue,
        onChange,
        finalValue: null,
    });
    const [opened, setOpened] = useState(false);
    const EmptyIcon = emptyIcon ?? TbPhotoOff;
    const [search, onSearchChange] = useInputState("");
    const [icons, setIcons] = useState<string[]>([]);
    const [searcher, setSearcher] = useState<Fuse<string> | null>(null);
    const searcherActive = searcher !== null;

    useEffect(() => {
        api.application.icons.all_icons().then((results) => {
            results.sort();
            setIcons(results);
            setSearcher(
                new Fuse(results, {
                    isCaseSensitive: false,
                    ignoreDiacritics: true,
                    shouldSort: true,
                    findAllMatches: true,
                    threshold: 0.2,
                }),
            );
        });
    }, [setIcons, setSearcher, searcherActive]);

    const [debouncedSearch] = useDebouncedValue(search, 0.2, { leading: true });

    const filteredIcons = useMemo(() => {
        if (debouncedSearch.length === 0) {
            return icons;
        }

        if (searcherActive) {
            return searcher
                .search(snakeCase(debouncedSearch))
                .map((v) => v.item);
        } else {
            return icons;
        }
    }, [debouncedSearch, icons.length, searcherActive]);

    return (
        <>
            <ActionIcon {...props} onClick={() => setOpened(true)}>
                {_value ? (
                    <DynamicIcon icon={_value} size={iconSize} />
                ) : (
                    <EmptyIcon opacity={0.4} size={iconSize} />
                )}
            </ActionIcon>
            <Modal
                opened={opened}
                onClose={() => setOpened(false)}
                zIndex={1000}
                centered
                size="xl"
                title={
                    <Group gap="sm">
                        <TbPhoto size={24} />
                        <Text>{t("components.iconSelector.title")}</Text>
                    </Group>
                }
                overlayProps={{
                    style: {
                        WebkitBackdropFilter: "blur(3px)",
                    },
                }}
            >
                <Stack gap="sm">
                    <Group wrap="nowrap" gap="sm">
                        <TextInput
                            value={search}
                            onChange={onSearchChange}
                            placeholder={t("components.iconSelector.search")}
                            leftSection={<TbSearch size={20} />}
                            size="md"
                            style={{ flexGrow: 1 }}
                        />
                        <Button
                            variant="subtle"
                            leftSection={<TbPhotoOff size={20} />}
                            color="red"
                            size="md"
                            justify="space-between"
                            onClick={() => handleChange(null)}
                        >
                            {t("components.iconSelector.clear")}
                        </Button>
                    </Group>
                    <Paper
                        p="sm"
                        radius="xs"
                        style={{ flexGrow: 1 }}
                        withBorder
                        shadow="xs"
                        h="50vh"
                    >
                        <SelectionContext.Provider
                            value={{
                                selected: _value ?? null,
                                setSelected: (name) => {
                                    handleChange(_value === name ? null : name);
                                },
                            }}
                        >
                            <VirtuosoGrid
                                className="icon-item-grid"
                                style={{ height: "100%" }}
                                onScroll={(e) =>
                                    console.log(
                                        (e.target as HTMLElement).scrollTop,
                                    )
                                }
                                totalCount={filteredIcons.length}
                                itemContent={(_, icon) => (
                                    <IconItem name={icon} />
                                )}
                                components={gridComponents}
                                data={filteredIcons}
                            />
                        </SelectionContext.Provider>
                    </Paper>
                </Stack>
            </Modal>
        </>
    );
}
