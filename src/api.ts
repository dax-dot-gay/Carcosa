import { TFunction } from "i18next";
import * as bindings from "../bindings";

export * from "../bindings";
export type * from "../bindings";

const api = bindings.createTauRPCProxy();

export default api;

export function writeError(
    error: bindings.SerializableError,
    t: TFunction<"translation", undefined>,
): string {
    switch (error.err) {
        case "no_active_project":
            return t("errors.formatted.noActiveProject");
        case "invalid_cast":
            return t("errors.formatted.invalidCast", {
                expected: error.context.expected_type,
                actual: JSON.stringify(error.context.value_type),
            });
        case "invalid_cast_datatype":
            return t("errors.formatted.invalidCastDatatype", {
                expected: JSON.stringify(error.context.value_type),
                actual: JSON.stringify(error.context.value),
            });
        default:
            return error.context;
    }
}
