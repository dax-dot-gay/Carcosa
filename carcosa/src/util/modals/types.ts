import { ModalProps } from "@mantine/core";
import { JSX } from "react";
import { IconType } from "react-icons";
import { ContextModalProps } from "@mantine/modals";

export type ModalRendererProps<P extends object = {}> = {context: ContextModalProps["context"]; id: string;} & P;
export type ModalRenderer<P extends object = {}> = (props: ModalRendererProps<P>) => JSX.Element;
export type ModalOptions<P extends object = {}> = {
    id: string;
    title: string;
    renderer: ModalRenderer<P>;
    icon?: IconType;
    settings?: Partial<Omit<ModalProps, "title" | "children">>;
};