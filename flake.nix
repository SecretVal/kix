{
  description = "SecretVal flake for kix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];

      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        final,
        ...
      }: let
        inherit (pkgs) dockerTools rustPlatform;
        inherit (dockerTools) buildImage;
        inherit (rustPlatform) buildRustPackage;
        name = "kix";
        version = "0.1.0";
      in {
        devShells = {
          default = pkgs.mkShell {
            inputsFrom = [self'.packages.default];
          };
        };

        packages = {
          default = buildRustPackage {
            inherit version;
            cargoHash = "sha256-29TD+xptdavbM7LQ0r9cfH95NaVdxu+/4zfvRmwSI48=";
            useFetchCargoVendor = true;
            pname = name;
            src = ./.;
          };

          docker = buildImage {
            inherit name;
            tag = version;
            config = {
              Cmd = "${self'.packages.default}/bin/${name}";
            };
          };
        };
      };
    };
}
