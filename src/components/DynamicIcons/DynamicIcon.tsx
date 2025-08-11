import api from "@/api";
import { Skeleton } from "@mantine/core";
import { useEffect, useState } from "react";
import { GenIcon, IconBaseProps, IconTree, IconType } from "react-icons";
import { TbQuestionMark } from "react-icons/tb";

function ResolvedIcon({
    data,
    ...props
}: { data: IconTree } & Partial<IconBaseProps>) {
    return GenIcon(data)(props);
}

export function DynamicIcon({
    icon,
    fallback,
    ...props
}: { icon: string; fallback?: IconType } & Partial<IconBaseProps>) {
    const size = props.size ?? 24;
    const [iconData, setIconData] = useState<IconTree | null | "loading">(
        "loading",
    );
    useEffect(() => {
        api.application.icons
            .icon(icon)
            .then((value) => {
                if (value) {
                    try {
                        const parsedIcon = JSON.parse(value) as IconTree;
                        setIconData(parsedIcon);
                    } catch (e) {
                        setIconData(null);
                    }
                } else {
                    setIconData(null);
                }
            })
            .catch(() => setIconData(null));
    }, [setIconData, icon, fallback]);
    const FallbackIcon = fallback ?? TbQuestionMark;

    return iconData === "loading" ? (
        <Skeleton height={size} width={size} animate={false} radius="xs" />
    ) : iconData === null ? (
        <FallbackIcon {...props} />
    ) : (
        <ResolvedIcon data={iconData} {...props} />
    );
}
