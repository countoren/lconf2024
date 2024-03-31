{ pkgs ? import <nixpkgs> {}
, lib ? pkgs.lib
, prefix ? "lconf"
}:
let commands = lib.fix (self: lib.mapAttrs pkgs.writeShellScript 
  {
      run-py = ''
        ${pkgs.python3}/bin/python main.py
      '';
      default = ''
        nix develop -c -- ${self.run-py}
      '';
  });
# try to run: nix-build commands.nix && ./result/bin/lconf-run-py 
in pkgs.symlinkJoin rec {
  name = prefix;
  passthru = lib.mapAttrs (name: command: pkgs.runCommand "${prefix}-${name}" {} ''
    mkdir -p $out/bin
    ln -sf ${command} $out/bin/${prefix}-${name}
    '') commands;
  paths = lib.attrValues passthru;
}
