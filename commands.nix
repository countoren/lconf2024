{ pkgs ? import <nixpkgs> {}
, prefix ? "b2n"
}:
let 
  #helpers
  file = stepNum: { filename, extension, focus?"" }:  ''
### ${filename}.${extension}
```${extension} ${focus}
${builtins.readFile ./step-${stepNum}/${filename}.${extension}}
```
  '';

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
      ${self.reveal} slides.md --highlight-theme github
    '';
    create-css = ''
      cp ${pkgs.writeTextFile {
        name = "slides.md"; 
        text = ''
        '';
      }} style.css

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

<style>
.code {
    width: 100%; /* Adjust the width as needed */
    margin: auto; /* Center the code block */
}
</style>
# From Bash To Nix

Note:  
- wellcome
- presenting myself

<!--s-->
## About Me

- 👋 **Hello everyone!** I am **Oren**, 
  You can find me on Github, Twitch, Youtube, Gmail as **CountOren**.
- 💻 **Experience:** 13 years of software engineering: 
  Nix, F#, C#, C++, Webs, Haskell, Rust, Bash, Linux tools
- 👷‍♂️ **Working For:** Carlson Software, We do:
  Mining Engineering, Civil Engineering, Embeeded Engineering...
- 🧑‍💻 Focused on **Nix** and functional programming to control them all
- 👨‍👧‍👦  Proud Husband and father of three.


<!--s-->
## Talk Road Map

- How to can use projects/exectuables as variables of FP language
- The Command Pattern
- Using the Command Pattern to Glue: NodeJS with F# and Rust

note: 
- what is bash? glue lang

<!--s-->
![nix](./images/glue-man.jpeg) <!-- .element width="65%" -->

note:
- what is nix? super glue lang
- bash being imprative harder to combine
- sequence of commands ,manual step in between like installing
- nix lets you define all the dependencies
- this called nixfying a project 
- where should i do my side effects in nix?
- all the commands into one command

<!--s-->
${file "1" {filename = "flake"; extension = "nix"; /*focus = "[|3-6,14|5-7]"; */}}
note:
- Flakes are the notice board and it gives a heads up what is in the project
- Flake allows us to use nix and its packages with a lock file 
- Flake allows you to describe an entire Git repository as a reliable collection of packages:
  - dev environments that contains dependecies related to your project 
      this is usually a good starting point when you try to "discover" a project 
  - fully nixified packages of the reporostiry
<!--v-->
${file "2" {filename = "flake"; extension = "nix"; }}
note:
- lets pick the nodejs eco system to start with 
<!--v-->
${file "2" {filename = "bash"; extension = "sh"; }}
note:
- here we run nix develop
- here we are assuming that the user that uses
our repo will know this commands exists
- and there are more then one command
- it will be nice to have them documented, or even better have them has one 
known command to serve as entry point
<!--v-->
${file "3" {filename = "server"; extension = "js"; }}
<!--s-->
${let step = file "3"; in ''
${step {filename = "flake"; extension = "nix"; }}
note:
- lets try to solve this by adding a nix default package to our flake
- this is a package that creates a shell script exectuable
- inside that script it contains the commands that we were executing manually before
- it is a bit meta 
- it will run nix develop 
- which will search a flake file in the current folder or in one of the parent folders 
and will try to run in the default devShell defined in it the node start server command
- lets see how the actual shell script looks like
---
- we can even add the package into our dev shell...
- when adding any package to the buildInputs it could be our default or the nodejs package 
  nix will append the bin folder to the PATH of the dev shell

- there is still an issue here we have infused 2 commands in one: node server.js and nix develop...
<!--v-->
${step {filename = "bash"; extension = "sh"; }}
note:
- nix build
- similar to nix develop it will search for the flake
- and will evualate and build the default package in it
- this process run in "closed" nix building enviorment 
- which allows fetching of source code,configuring, patching, building
- this build process must end by creating a folder or a file in nix store
- when done by default it will create for us a result symlink to this file or folder in the current folder
---
''}
<!--s-->
${let step = file "4"; in ''
${step {filename = "flake"; extension = "nix";}}
<!--v-->
${step {filename = "bash"; extension = "sh"; }}
''}
<!--s-->
${let step = file "5"; in ''
${step {filename = "commands"; extension = "nix"; }}
<!--v-->
${step {filename = "flake"; extension = "nix"; }}
<!--v-->
${step {filename = "bash"; extension = "sh"; }}
''}
<!--s-->
${let step = file "6"; in ''
${step {filename = "commands"; extension = "nix"; }}
<!--v-->
${step {filename = "flake"; extension = "nix"; }}
<!--v-->
${step {filename = "bash"; extension = "sh"; }}
''}
<!--s-->
${let step = file "7"; in ''
${step {filename = "commands"; extension = "nix"; }}
<!--v-->
${step {filename = "flake"; extension = "nix"; }}
<!--v-->
${step {filename = "bash"; extension = "sh"; }}
''}
<!--s-->
${let step = file "8"; in ''
${step {filename = "commands"; extension = "nix"; }}
<!--v-->
${step {filename = "bash"; extension = "sh"; }}
<!--v-->
${step {filename = "flake"; extension = "nix"; }}
''}
<!--s-->
# Live Coding
![live-coding](./images/live-coding.jpeg) <!-- .element width="90%" -->
<!--s-->
# Q & A
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
