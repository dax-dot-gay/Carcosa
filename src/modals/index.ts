import openCreateProjectModal from "./CreateProjectModal";

export function useModals() {
    return {
        createProject: openCreateProjectModal,
    };
}

export type ModalsType = ReturnType<typeof useModals>;
