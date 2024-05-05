{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  commands = import ./commands.nix { inherit pkgs; };
  in
  {
    packages.${system} = {
      start = commands.bin.start;
      editor = pkgs.writeShellScriptBin "editor" "nix develop -c neovide";
      default = commands;
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        reveal-md
        commands

        playwright
        carbon-now-cli
        (vims.createNvim {
          inherit pkgs; 
          pkgsPath = ".";
          fontSize = "24";

          # on vifm:
          # :set viewcolumns=-{name}
          # :set statusline=_
          additionalVimrc =  '' 
            let g:neovide_fullscreen = v:true
          '';
          additionalPlugins = with pkgs.vimPlugins; [

          ];
        })
      ];
    };

  };
}
