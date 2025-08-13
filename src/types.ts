export type DeepPartial<T extends object> = Partial<{
    [key in keyof T]: T[key] extends object ? DeepPartial<T[key]> : T[key];
}>;
