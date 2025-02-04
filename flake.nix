{
description = ''Notedeck flake setup with nixvim'';

inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    nixvim.url = "github:Pleb5/neovim-flake/master";
};

outputs = { self, nixpkgs, flake-utils, nixvim, ... }:

    flake-utils.lib.eachDefaultSystem (system:
        let
            pkgs = nixpkgs.legacyPackages.${system};
            nvim = nixvim.packages.${system}.nvim;

            # Read environment variables instead of using `--arg`
            # the ' == "true ' will be evaluated to false if the variable does not exist
            # which follows from ' "" == true ' actually evaluating to false
            use_android = builtins.getEnv "use_android" == "true";
            android_emulator = builtins.getEnv "android_emulator" == "true";

            # Import the repo's shell.nix
            repoShell = import ./shell.nix { inherit pkgs use_android android_emulator; };
        in {
            devShell = pkgs.mkShell {
                buildInputs = repoShell.nativeBuildInputs 
                        ++ repoShell.buildInputs ++ [ 
                    nvim
                    pkgs.ripgrep
                    pkgs.cargo
                    pkgs.rustc
                ];
                shellHook = ''
                    export PATH="$HOME/.cargo/bin:$PATH"
                    export LD_LIBRARY_PATH="${repoShell.LD_LIBRARY_PATH}:$LD_LIBRARY_PATH"
                    export XDG_SESSION_TYPE=wayland
                    export WAYLAND_DISPLAY=wayland-0
                    echo "Welcome to your dev shell for Notedeck!"
                '';
            };
        }
    );    
}
