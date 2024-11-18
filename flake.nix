{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        inherit (nixpkgs) lib;

        pkgs = nixpkgs.legacyPackages.${system};
        rpath = lib.makeLibraryPath (with pkgs; [
          fontconfig
          wayland
        ]);
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "hybrid-bar";
          inherit ((lib.importTOML (self + "/Cargo.toml")).package) version;

          src = self;

          cargoLock.lockFile = self + "/Cargo.lock";

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            gtk-layer-shell
            gtk3
          ];

          postFixup = ''
            patchelf $out/bin/hybrid-bar --add-rpath ${rpath}
          '';
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            gtk-layer-shell
            gtk3
            pkg-config
          ];

          LD_LIBRARY_PATH = rpath;
        };
      }
    );
}