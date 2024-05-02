{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  in
  {
    packages.${system}.default = pkgs.writeShellScriptBin "run" ''
          nix develop -c -- node server.js
    '';
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        nodejs
        self.packages.${system}.default
      ];
    };

  };
}
