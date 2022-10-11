{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs, ... }:
    let 
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      defaultPackage.${system} = pkgs.rustPlatform.buildRustPackage {
        pname = "psi-shell"; 
        version = "0.1.0";
        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          gtk3
          gtk-layer-shell
        ];
      };

      devShell.${system} = import ./shell.nix { inherit pkgs; };
    };
}
