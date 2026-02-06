import ReactDOM from "react-dom/client";
import { App } from "./App";
import "@fontsource-variable/atkinson-hyperlegible-mono";
import "@fontsource-variable/atkinson-hyperlegible-mono/wght-italic.css";
import "@fontsource-variable/atkinson-hyperlegible-next";
import "@fontsource-variable/atkinson-hyperlegible-next/wght-italic.css";

import "@mantine/core/styles.layer.css";
import "@mantine/charts/styles.layer.css";
import "@mantine/notifications/styles.layer.css";
import "@mantine/spotlight/styles.layer.css";
import "@mantine/dropzone/styles.layer.css";
import "@mantine/tiptap/styles.layer.css";
import "mantine-contextmenu/styles.layer.css";
import "./styles/index.scss";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <App />,
);
