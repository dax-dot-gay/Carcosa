import { createTheme } from "@mantine/core";

const PlaceholderTheme = createTheme({
    colors: {
        dark: [
            "#cdd6f4",
            "#9399b2",
            "#7f849c",
            "#6c7086",
            "#585b70",
            "#45475a",
            "#313244",
            "#1e1e2e",
            "#181825",
            "#11111b",
        ],
        light: [
            "#eff1f5",
            "#e4e5e7",
            "#c6c9d1",
            "#a5acbb",
            "#8992a7",
            "#77829c",
            "#6d7b98",
            "#5c6985",
            "#505d77",
            "#43506b",
        ],
        yellow: [
            "#fff8e4",
            "#fdefd0",
            "#f9e2af",
            "#f4cb71",
            "#f0bb47",
            "#eeb12d",
            "#eeac1d",
            "#d39610",
            "#bc8506",
            "#a37200",
        ],
    },
    primaryColor: "yellow",
    primaryShade: { dark: 2, light: 6 },
    fontFamily: "Atkinson Hyperlegible Next Variable",
    fontFamilyMonospace: "Atkinson Hyperlegible Mono Variable",
    autoContrast: true,
});

export { PlaceholderTheme };
