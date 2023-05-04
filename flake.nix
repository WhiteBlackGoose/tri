{
  description = "TRI editor project";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = inputs@{ nixpkgs, ... }: {
    devShells.x86_64-linux.default =
      with nixpkgs.legacyPackages.x86_64-linux;
      mkShell {
        buildInputs = [
          cargo
          rustc
          rust-analyzer
          vscode-extensions.vadimcn.vscode-lldb
          imagemagick
        ];
        VSCODE_CODELLDB = "${nixpkgs.vscode-extensions.vadimcn.vscode-lldb}";
      };
  };
}
