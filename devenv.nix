{
    pkgs,
    inputs,
    ...
}:

{
    packages = with pkgs; [
        git
        cargo-autoinherit
        webkitgtk_4_1
        nixgl.auto.nixGLDefault
        zellij
    ];
    overlays = [
        inputs.nixgl.overlays.default
    ];
    languages = {
        rust.enable = true;
        javascript = {
            enable = true;
            pnpm.enable = true;
        };
        typescript.enable = true;
    };
    scripts.dev.exec = ''
        zellij -l dev.kdl
    '';
    scripts.install.exec = ''
        cd carcosa
        pnpm install
    '';
    files."dev.kdl".text = ''
        layout {
            pane split_direction="vertical"  {
                pane {
                    command "nixGL"
                    args "pnpm" "tauri" "dev" "--no-watch"
                    cwd "carcosa"
                }
                pane focus=true cwd="carcosa"
            }
        }
    '';
}
