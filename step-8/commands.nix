{ pkgs ? import <nixpkgs> {}
, prefix ? "lconf"
}:
let commands = pkgs.lib.fix (self: pkgs.lib.mapAttrs pkgs.writeShellScript
{
    welcome = ''
      ${pkgs.figlet}/bin/figlet '${prefix} dev shell'
      echo 'press lconf-<TAB><TAB> to see all the commands'
    '';
    start-server = ''
        node server.js
    '';
    default = ''
        nix develop -c -- ${self.start-server}
    '';
});
in pkgs.symlinkJoin rec {
  name = prefix;
  passthru.set = commands;
  /*
$ nix repl '<nixpkgs>'
nix-repl> runCommand                              
«lambda @ /nix/store/fjv2fjy30lcf0g4rcyqrky7mxl5zy7yj-4fvp92iazfdj73g03wvnk8451mihhai5-source/pkgs/build-support/trivial-builders/default.nix:14:16»
  # See https://nixos.org/manual/nixpkgs/unstable/#trivial-builder-runCommand
  */
  passthru.bin = pkgs.lib.mapAttrs (name: command: pkgs.runCommand "${prefix}-${name}" {} '' 
    mkdir -p $out/bin
    ln -sf ${command} $out/bin/${
        if name == "default" then prefix else prefix+"-"+name
    }
  '') commands;
  paths = pkgs.lib.attrValues passthru.bin;
}
