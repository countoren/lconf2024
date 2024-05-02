{ pkgs ? import <nixpkgs> {}
}:
pkgs.rustPlatform.buildRustPackage {
  name = "rust-cli";
  src = ./.;
  # cargoHash = "";
  cargoHash = "sha256-RteKuN1eRIWFPBfT02qx7aJUxKbzHT/OV2zzyDUlrCo=";
}
