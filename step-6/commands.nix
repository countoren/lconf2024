{ pkgs ? import <nixpkgs> {} }:
let commands = pkgs.lib.fix (self: pkgs.lib.mapAttrs pkgs.writeShellScript
{
    start-server = ''
        node server.js
    '';
    default = ''
        nix develop -c -- ${self.start-server}
    '';
});
in pkgs.lib.mapAttrs (name: command: pkgs.writeShellScriptBin name command) commands
