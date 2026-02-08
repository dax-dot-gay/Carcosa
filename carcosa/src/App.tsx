import { MantineProvider } from "@mantine/core";
import { PlaceholderTheme } from "./util/themes/theme";
import { Notifications } from "@mantine/notifications";
import { RouterProvider } from "react-router";
import { AppRouter } from "./views/routes";
import { ApplicationModals } from "./util/modals";

export function App() {
    return (
        <MantineProvider theme={PlaceholderTheme} forceColorScheme="dark">
            <ApplicationModals>
                <Notifications />
                <RouterProvider router={AppRouter} />
            </ApplicationModals>
        </MantineProvider>
    );
}
