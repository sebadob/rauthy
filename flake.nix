{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    systems.url = "github:nix-systems/default";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachSystem
      [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ]
      (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          # rust-toolchain.toml pins what CI's builder image compiles with (the
          # base image of Dockerfile_builder).
          rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          # Editor tooling for the dev shell only. Same version as rustc, so
          # rust-analyzer's proc-macro server matches the compiler.
          rust-dev = rust.override {
            extensions = [
              "rust-analyzer"
              "rust-src"
            ];
          };
        in
        {
          formatter = pkgs.nixfmt;

          # mkShell itself brings the C toolchain, make, tar and the coreutils
          # that the -sys crates and the recipes need. Container runtimes and
          # git stay on the host.
          devShells.default = pkgs.mkShell {
            name = "rauthy-dev";
            packages = [
              rust-dev
              pkgs.cargo-nextest
              pkgs.just
              # openssl-src, the vendored OpenSSL, configures its build with it.
              pkgs.perl
              pkgs.pkg-config
              # The justfile's node_image: `npm` for the frontend recipes.
              pkgs.nodejs_22
              # `just build-wasm`; its wasm32 target is in rust-toolchain.toml.
              pkgs.wasm-pack
              # The test recipes poll the backend and Postgres with it.
              pkgs.curl
              # The justfile looks up the container runtime with it.
              pkgs.which
              # `clear`, the first command of most recipes.
              pkgs.ncurses
            ];
            # A debug build compiles tikv-jemalloc-sys with -O0, where glibc
            # warns that _FORTIFY_SOURCE needs optimization; jemalloc's
            # configure runs its checks with -Werror and fails on the warning.
            hardeningDisable = [ "fortify" ];
          };
        }
      );
}
