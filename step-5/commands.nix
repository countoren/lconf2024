{ pkgs ? import <nixpkgs> {} }:
pkgs.lib.fix (self: pkgs.lib.mapAttrs 
  (key: value: pkgs.writeShellScript key value)
{
    start-server = ''
        node server.js
    '';
    default = ''
        nix develop -c -- ${self.start-server}
    '';
})
/*
https://noogle.dev/f/lib/mapAttrs
mapAttrs (name: value: name + "-" + value)
   { x = "foo"; y = "bar"; }
=> { x = "x-foo"; y = "y-bar"; }


https://noogle.dev/f/lib/fix
fix (self: 
{ 
  foo = "foo"; 
  bar = "bar"; 
  foobar = self.foo + self.bar; 
})
=> { bar = "bar"; foo = "foo"; foobar = "foobar"; }

*/
