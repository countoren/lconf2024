{ pkgs ? import <nixpkgs> {}
}:
/*
$ nix-build
$ ./result/bin/rust-cli
Hello, world!
*/
pkgs.rustPlatform.buildRustPackage {
  name = "rust-cli";
  src = ./.;
  # cargoHash = "";
  cargoHash = "sha256-RteKuN1eRIWFPBfT02qx7aJUxKbzHT/OV2zzyDUlrCo=";
}
