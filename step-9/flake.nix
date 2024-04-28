{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  commands = import ./commands.nix { inherit pkgs; };
  in
  {
    packages.${system} = {
      rust-project = import ./rust-project { inherit pkgs;};
      default = commands;
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        nodejs
        commands

        cargo
        rustc
        evcxr
      ];
      shellHook = commands.set.welcome;
    };
  };
}
