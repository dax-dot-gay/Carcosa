import {ApplicationModals} from "./Provider";
import renderers from "./renderers";

export function openModal<T extends keyof (typeof renderers)["openers"]>(modal: T, props: Parameters<(typeof renderers)["openers"][T]>[0]): string {
    return renderers.openers[modal](props);
}

export {ApplicationModals};