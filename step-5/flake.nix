{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  in
  {
    packages.${system} = {
      # try run: nix run .#run-py to get result
      run-py = pkgs.writeShellScriptBin "run-py" ''
        ${pkgs.python3}/bin/python main.py
      '';
      # try run: nix run
      default = pkgs.writeShellScriptBin "run" ''
        nix develop -c -- ${self.packages.${system}.run-py}/bin/run-py
      '';
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        python3
      ];
    };
  };
}
