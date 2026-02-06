import { createMemoryRouter } from "react-router";
import { StartupView } from "./startup";

const AppRouter = createMemoryRouter(
    [
        {
            path: "/startup",
            element: <StartupView />,
        },
    ],
    { initialEntries: ["/startup"], initialIndex: 0 },
);

export { AppRouter };
