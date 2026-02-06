import { MantineProvider } from "@mantine/core";
import { PlaceholderTheme } from "./util/themes/theme";
import { ModalsProvider } from "@mantine/modals";
import { Notifications } from "@mantine/notifications";
import { RouterProvider } from "react-router";
import { AppRouter } from "./views/routes";

export function App() {
    return (
        <MantineProvider theme={PlaceholderTheme} forceColorScheme="dark">
            <ModalsProvider>
                <Notifications />
                <RouterProvider router={AppRouter} />
            </ModalsProvider>
        </MantineProvider>
    );
}
