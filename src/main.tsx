import ReactDOM from "react-dom/client";
import "@mantine/core/styles.css";
import "@mantine/dates/styles.css";
import "@mantine/dropzone/styles.css";
import "@mantine/notifications/styles.css";
import "@mantine/spotlight/styles.css";
import "@mantine/tiptap/styles.css";
import "@gfazioli/mantine-split-pane/styles.css";
import { LocalizationProvider } from "./localization";
import { BrowserRouter, Route, Routes } from "react-router";
import { LayoutView } from "./ui/layout/Layout";
import { LandingPage } from "./ui/landing/LandingPage";
import { Wrapper } from "./ui/Wrapper";
import "@/theme/style.css";
import { InitProvider } from "./context/init/InitProvider";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <LocalizationProvider>
        <InitProvider>
            <BrowserRouter>
                <Routes>
                    <Route path="/" element={<Wrapper />}>
                        <Route index element={<LandingPage />} />
                        <Route path="/project" element={<LayoutView />}></Route>
                    </Route>
                </Routes>
            </BrowserRouter>
        </InitProvider>
    </LocalizationProvider>
);
