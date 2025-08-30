import { NodeDesc } from "@/api";
import * as Common from "./common";
import { IconType } from "react-icons";
import {
    TbArrowAutofitContentFilled,
    TbFileDescription,
    TbForms,
    TbInfoCircleFilled,
    TbInfoTriangleFilled,
    TbLabelFilled,
    TbLayoutBottombarCollapseFilled,
    TbLayoutColumns,
    TbLayoutFilled,
    TbLetterCase,
    TbLink,
    TbLinkPlus,
    TbNumbers,
    TbSelect,
    TbSquare,
    TbToggleRightFilled,
} from "react-icons/tb";

export const NODE_CATEGORIES: NodeDesc["node_category"][] = [
    "field",
    "container",
    "other",
];

export const NODE_TYPES: {
    [key in NodeDesc["node_category"]]: Extract<
        NodeDesc,
        { node_category: key }
    >["node_kind"][];
} = {
    other: ["text", "alert"],
    container: [
        "collapsible",
        "columns",
        "flexible_row",
        "labelled_group",
        "wrapper",
    ],
    field: [
        "linked_document",
        "multi_linked_documents",
        "multi_select",
        "number_field",
        "rich_text",
        "single_select",
        "switch",
        "text_field",
    ],
};

export const NODE_CATEGORY_ICONS: {
    [key in NodeDesc["node_category"]]: IconType;
} = {
    other: TbInfoCircleFilled,
    container: TbLayoutFilled,
    field: TbForms,
};

export const NODE_KIND_ICONS: {
    [category in NodeDesc["node_category"]]: {
        [kind in Extract<
            NodeDesc,
            { node_category: category }
        >["node_kind"]]: IconType;
    };
} = {
    other: {
        text: TbLetterCase,
        alert: TbInfoTriangleFilled,
    },
    container: {
        columns: TbLayoutColumns,
        collapsible: TbLayoutBottombarCollapseFilled,
        labelled_group: TbLabelFilled,
        wrapper: TbSquare,
        flexible_row: TbArrowAutofitContentFilled,
    },
    field: {
        text_field: TbForms,
        number_field: TbNumbers,
        switch: TbToggleRightFilled,
        single_select: TbSelect,
        multi_select: TbSelect,
        rich_text: TbFileDescription,
        linked_document: TbLink,
        multi_linked_documents: TbLinkPlus,
    },
};

export { Common };
