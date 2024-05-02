{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  in
  {
    packages.${system} = {
      default = import ./commands.nix { inherit pkgs; };
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        nodejs
        self.packages.${system}.default.bin.start-server
      ];
    };
  };
}
