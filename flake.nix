{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs";
  inputs.vims = {
    url = "github:countoren/vims";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, vims }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  commands = import ./commands.nix { inherit pkgs; };
  step9Commands = import ./step-9/commands.nix { inherit pkgs; serverIp = "localhost"; };
  in
  {
    packages.${system} = {
      client = pkgs.writeShellScriptBin "client" ''
        ${step9Commands.set.server-get} | ${import ./step-9/rust-project { inherit pkgs; }}/bin/rust-cli
      '';
      start = commands.bin.start;
      editor = pkgs.writeShellScriptBin "editor" "nix develop -c neovide";
      default = commands;
    };

    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        reveal-md
        commands

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
