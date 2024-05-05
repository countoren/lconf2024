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
  Nix, F#, C#, C++, Web, Haskell, Rust, Bash, Linux tools
- 👷‍♂️ **Working For:** Carlson Software, We do:
  Mining Engineering, Civil Engineering, Embeeded Engineering...
- 🧑‍💻 Focused on **Nix** and functional programming to control them all
- 👨‍👧‍👦  Proud Husband and father of three.


<!--s-->
## Talk Road Map

- Using Projects/Executables as Variables in FP Language
- Discovering The Command Pattern while breaking down some Nix Fundamentals
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
- how should i do my side effects in nix?
- all the commands into one command

${"" /* import ./code-slides.nix */}

<!--s-->
# Lets See Some Code
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
