{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  commands = import ./commands.nix { inherit pkgs; };
  in
  {
    packages.${system}.default = commands.default;
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        python3
        commands
      ];
    };
  };
}
