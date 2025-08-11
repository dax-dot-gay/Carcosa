import { Skeleton } from "@mantine/core";
import { useContext, useEffect, useMemo } from "react";
import { GenIcon, IconBaseProps, IconTree, IconType } from "react-icons";
import { TbQuestionMark } from "react-icons/tb";
import { IconsContext } from "./util";

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
    const { icons, request, unrequest } = useContext(IconsContext);
    const size = props.size ?? 24;
    useEffect(() => {
        request(icon);
        return () => unrequest(icon);
    }, [icon]);
    const FallbackIcon = fallback ?? TbQuestionMark;
    const iconData = useMemo(() => {
        const result = icons[icon];
        return result === undefined ? "loading" : result;
    }, [icon, icons]);

    return iconData === "loading" ? (
        <Skeleton height={size} width={size} animate={false} radius="xs" />
    ) : iconData === null ? (
        <FallbackIcon {...props} />
    ) : (
        <ResolvedIcon data={iconData} {...props} />
    );
}
