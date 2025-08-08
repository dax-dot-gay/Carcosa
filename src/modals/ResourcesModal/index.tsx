import { TbCategoryFilled } from "react-icons/tb";
import { createModalOpener } from "../common";
import { ResourceModalOpen } from "./types";
import { ResourcesModal } from "./modal";

const openResourcesModal = createModalOpener({
    id: "resources",
    title: "modals.resources.title",
    icon: TbCategoryFilled,
    renderer: (props: { mode: ResourceModalOpen }) => (
        <>
            <ResourcesModal {...props} />
        </>
    ),
    size: "75vw",
    overlayProps: {
        style: {
            WebkitBackdropFilter: "blur(3px)",
        },
    },
    mah: "90vh",
    centered: true,
});

export type * from "./types";
export default openResourcesModal;
