{
  description = "Flyte";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixos-unstable.url = "nixpkgs/nixos-unstable-small";
    flake-utils.url = "github:numtide/flake-utils";
    nixlib.url = "github:nix-community/nixpkgs.lib";
    nixify = {
      url = "github:rvolosatovs/nixify";
      inputs.nixlib.follows = "nixlib";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    wit-deps = {
      url = "github:bytecodealliance/wit-deps/v0.4.0";
      inputs.nixlib.follows = "nixlib";
      inputs.nixify.follows = "nixify";
    };
    # Needed due to wit-deps not existing in nixpkgs.
    # TODO: Remove once wit-deps is in nixpkgs or move to fenix? maybe?
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem =
        { system, ... }:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
          };
        in
        {
          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              pkg-config
              openssl.dev
              protobuf

              rustToolchain
            ];
          };
        };
    };
}
