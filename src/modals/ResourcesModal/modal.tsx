import { ResourceModalOpen } from "./types";
import "./style.scss";
import { IndexView } from "./views/index/IndexView";
import { LocalRouter, Route } from "@/context/routing";
import { ModalLayout } from "./views/layout/ModalLayout";

export function ResourcesModal({ mode }: { mode: ResourceModalOpen }) {
    return (
        <LocalRouter
            routerId="resources"
            initialPath="/"
            fallback={() => <>ERROR!</>}
        >
            <Route path="/" element={ModalLayout}>
                <Route path="/" element={IndexView} />
            </Route>
        </LocalRouter>
    );
}
