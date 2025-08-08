import openCreateProjectModal from "./CreateProjectModal";
import openResourcesModal from "./ResourcesModal";

export function useModals() {
    return {
        createProject: openCreateProjectModal,
        resourceManager: openResourcesModal,
    };
}

export type ModalsType = ReturnType<typeof useModals>;
