import { ReactNode } from "react";
import i18next from "i18next";
import { I18nextProvider } from "react-i18next";
import * as langEn from "./lang/en.json";

const instance = i18next.createInstance({
    fallbackLng: "en",
    lng: "en",
    interpolation: {
        escapeValue: false,
    },
    resources: {
        en: {
            translation: langEn,
        },
    },
});

instance.init();

export function LocalizationProvider({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    return <I18nextProvider i18n={instance}>{children}</I18nextProvider>;
}
