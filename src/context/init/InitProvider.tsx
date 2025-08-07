import { shadcnCssVariableResolver } from "@/theme/cssVariableResolver";
import { shadcnTheme } from "@/theme/theme";
import {
    Center,
    Group,
    MantineProvider,
    Stack,
    Text,
    Title,
} from "@mantine/core";
import { ReactNode, Suspense, use, useState } from "react";
import AppIcon from "@/assets/icon.svg?react";
import { useTranslation } from "react-i18next";
import { InitContext } from "./types";
import api, { State } from "@/api";
import { useEvent } from "@/events";

function InitFallback() {
    const { t } = useTranslation();
    return (
        <MantineProvider
            theme={shadcnTheme}
            cssVariablesResolver={shadcnCssVariableResolver}
            forceColorScheme="dark"
        >
            <Center id="fallback-loader" w="100vw" h="100vw">
                <Group gap="md">
                    <AppIcon id="landing-page-icon" />
                    <Stack gap={0} justify="start" align="start">
                        <Title order={2} ff="monospace" fw={400}>
                            {t("app.name")}
                        </Title>
                        <Text size="sm" c="dimmed" ff="monospace">
                            {t("app.subtitle")}
                        </Text>
                    </Stack>
                </Group>
            </Center>
        </MantineProvider>
    );
}

export function InitProvider({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    return (
        <Suspense fallback={<InitFallback />}>
            <InitProviderInner>{children}</InitProviderInner>
        </Suspense>
    );
}

const FULL_STATE_PROMISE = api.application.full_state();

function InitProviderInner({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    const initialState = use(FULL_STATE_PROMISE);
    const [state, setState] = useState<State>(initialState);

    useEvent(
        api.application.updated_state,
        (newState) => {
            setState(newState);
        },
        [setState]
    );

    return (
        <InitContext.Provider value={{ state: state }}>
            {children}
        </InitContext.Provider>
    );
}
