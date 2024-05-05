{ pkgs ? import <nixpkgs> {}
, prefix ? "lconf"
, serverIp ? "localhost"
}:
let commands = pkgs.lib.fix (self: pkgs.lib.mapAttrs pkgs.writeShellScript
{
    welcome = ''
      ${pkgs.figlet}/bin/figlet '${prefix} dev shell'
      echo 'press lconf-<TAB><TAB> to see all the commands'
    '';

    server-start = ''
      ${pkgs.nodejs}/bin/node server.js
    '';
    server-build-and-start = ''
      ${self.fable-build-js} && ${self.server-start}
    '';
    server-get = ''
      ${pkgs.curl}/bin/curl ${serverIp}:3000
    '';

    fable-build = ''
      ${pkgs.fable}/bin/fable $@
    '';

    fable-build-js = ''
       ${self.fable-build} -e ".js" model.fsx 
    '';

    fable-build-rs = ''
        cp -f ${pkgs.runCommand "model-rs" {} ''
          ln -sf ${./model.fsx} model.fsx && \
          ${self.fable-build} --lang rust -o $out -e ".rs" model.fsx
        ''}/model.rs $1
    '';

    repo-path = "${pkgs.git}/bin/git rev-parse --show-toplevel";

    rust-cli-transpile = ''
      ${self.fable-build-rs} $(${self.repo-path})/step-9/rust-project/src/model.rs 
    '';

    rust-cli-run-info = ''
       echo 'this is a shell with "side effect" assuming rust-project is defined correctly on the flake level.'
    '';
    rust-cli-run = ''
      ${self.rust-cli-transpile} && nix run .#rust-project
    '';

    rust-cli-get-from-server = ''
      ${self.server-get} | ${self.rust-cli-run}
    '';


    rust-cli-test = ''
      echo '{"ecoSystem":"JS","text":"Hello World"}' | ${self.rust-cli-run}
    '';
    default = "${self.rust-cli-get-from-server}";
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
