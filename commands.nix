{ pkgs ? import <nixpkgs> {}
, prefix ? "b2n"
}:
let 
  commands = pkgs.lib.fix (self: pkgs.lib.mapAttrs pkgs.writeShellScript
{
    welcome = ''
      ${pkgs.figlet}/bin/figlet 'From Nix To Bash'
      echo 'press ${prefix}-<TAB><TAB> to see all the commands'
    '';

    # Dependencies
    reveal = "${pkgs.reveal-md}/bin/reveal-md $@";
    eza = "${pkgs.eza}/bin/eza $@";
    gramma = "${pkgs.nodePackages.gramma}/bin/gramma $@";
    entr = "${pkgs.entr}/bin/entr $@";


    # Commands
    checkSpelling = ''
      ${self.gramma} check -m commands.nix
    '';

    start = ''
      ${self.create-slides} && \
      ${self.reveal} slides.md  --highlight-theme github
    '';
    create-slides = ''
      cp ${pkgs.writeTextFile {
        name = "slides.md"; 
        text = ''
---
title: From Bash To Nix
theme: league
separator: <!--s-->
verticalSeparator: <!--v-->
---

# From Bash To Nix

Note: 
- wellcome
- presenting myself
- what is bash? glue lang


<!--s-->
![ls](./images/glue-man.jpeg) <!-- .element width="65%" -->

Note:
- what is nix? super glue lang
- what is the problem with bash only
- Without nix it is a lot harder to leverage different tools from different enviornoments.

<!--s-->
### flake.nix
```nix
${builtins.readFile ./step-1/flake.nix}
```
Notes:
- describe flake 
- Flake allows us to use nix and its packages with a lock file 
- Flake allows you to describe an entire Git repository as a reliable collection of packages:
  - dev environments that contains dependecies related to your project 
      this is usually a good starting point when you try to "discover" a project 

  - definition of the current project, its dependencies and how to build  
  - flake notice board and it gives a heads up what is in the project
<!--v-->
### flake.nix
```nix
${builtins.readFile ./step-2/flake.nix}
```
<!--v-->
```console
${builtins.readFile ./step-2/bash.sh}
```
<!--v-->
```js
${builtins.readFile ./step-3/server.js}
```
<!--v-->

![ls](./images/i.png)
<!--s-->
```nix
${builtins.readFile ./step-3/flake.nix}
```
<!--v-->
```console
${builtins.readFile ./step-3/bash.sh}
```
<!--s-->
```nix
${builtins.readFile ./step-4/flake.nix}
```
<!--v-->
```console
${builtins.readFile ./step-4/bash.sh}
```
<!--s-->
```nix
${builtins.readFile ./step-5/commands.nix}
```
<!--v-->
```nix
${builtins.readFile ./step-5/flake.nix}
```
<!--v-->
```console
${builtins.readFile ./step-5/bash.sh}
```
<!--s-->
```nix
${builtins.readFile ./step-6/commands.nix}
```
<!--v-->
```nix
${builtins.readFile ./step-6/flake.nix}
```
<!--v-->
```console
${builtins.readFile ./step-6/bash.sh}
```
<!--s-->
```nix
${builtins.readFile ./step-7/commands.nix}
```
<!--v-->
```nix
${builtins.readFile ./step-7/flake.nix}
```
<!--v-->
```console
${builtins.readFile ./step-7/bash.sh}
```
<!--s-->
```nix
${builtins.readFile ./step-8/commands.nix}
```
<!--v-->
```console
${builtins.readFile ./step-8/bash.sh}
```
<!--v-->
```nix
${builtins.readFile ./step-8/flake.nix}
```
<!--s-->
# Demo Time
<!--s-->
```nix
${builtins.readFile ./step-9/commands.nix}
```
<!--v-->
```nix
${builtins.readFile ./step-9/flake.nix}
```
        '';
      }} slides.md
  '';

    default = "ls commands.nix | ${self.entr} -r nix run .#start";
});
in pkgs.symlinkJoin rec {
  name = prefix;
  passthru.set = commands;
  passthru.bin = pkgs.lib.mapAttrs (name: command: pkgs.runCommand "${prefix}-${name}" {} '' 
    mkdir -p $out/bin
    ln -sf ${command} $out/bin/${
        if name == "default" then prefix else prefix+"-"+name
    }
  '') commands;
  paths = pkgs.lib.attrValues passthru.bin;
}
