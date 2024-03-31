{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  in
  {
    packages.${system} = {
      # try run: nix run .#run-py to get python not exists error
      run-py = pkgs.writeShellScriptBin "run-py" ''
        python main.py
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
