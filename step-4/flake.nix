{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  in
  {
    packages.${system} = {
      start-server = pkgs.writeShellScript "run-server" ''
          node server.js
      '';
      # nix run should still start the service
      default = pkgs.writeShellScriptBin "run" ''
          nix develop -c -- ${self.packages.${system}.start-server}
      '';
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        nodejs
        # this wont work anymore becuase it is not a folder that contains bin with exectuable
        # self.packages.${system}.start-server
      ];
    };

  };
}
