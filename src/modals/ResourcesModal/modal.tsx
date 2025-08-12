import { ResourceModalOpen } from "./types";
import "./style.scss";
import { IndexView } from "./views/index/IndexView";
import { LocalRouter, Route } from "@/context/routing";
import { ModalLayout } from "./views/layout/ModalLayout";
import { TemplateEditor } from "./views/template_editor/TemplateEditor";
import { TemplateCreator } from "./views/template_create_form/TemplateCreator";

export function ResourcesModal({ mode }: { mode: ResourceModalOpen }) {
    return (
        <LocalRouter
            routerId="resources"
            initialPath={
                mode === "manager"
                    ? "/"
                    : mode === "createTemplate"
                      ? "/templates/create"
                      : "/"
            }
            fallback={() => <>ERROR!</>}
        >
            <Route path="/" element={ModalLayout}>
                <Route path="/" element={IndexView} />
                <Route
                    path="/templates/:mode(edit|view)/:id?"
                    element={TemplateEditor}
                />
                <Route path="/templates/create" element={TemplateCreator} />
            </Route>
        </LocalRouter>
    );
}
