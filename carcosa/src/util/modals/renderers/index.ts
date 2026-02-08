import CreateProject from "./CreateProject";

export default {
    renderers: {
        create_project: CreateProject.renderer
    },
    openers: {
        create_project: CreateProject.opener
    }
};