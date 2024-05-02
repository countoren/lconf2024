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
### flake.nix
```nix 
{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  in
  {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
      ];
    };
  };
}

```

note:
- Flakes are the notice board and it gives a heads up what is in the project
- Flake allows us to use nix and its packages with a lock file 
- Flake allows you to describe an entire Git repository as a reliable collection of packages:
  - dev environments that contains dependecies related to your project 
      this is usually a good starting point when you try to "discover" a project 
  - fully nixified packages of the reporostiry
<!--v-->
### flake.nix
```nix 
{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  in
  {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        nodejs
      ];
    };

  };
}

```

note:
- lets pick the nodejs eco system to start with 
<!--v-->
### bash.sh
```sh 
$ nix develop
# create server.js
$ node server.js
# check browser at localhost:3000

```

note:
- here we run nix develop
- here we are assuming that the user that uses
our repo will know this commands exists
- and there are more then one command
- it will be nice to have them documented, or even better have them has one 
known command to serve as entry point
<!--v-->
### server.js
```js 
const { createServer } = require('node:http');

const hostname = '127.0.0.1';
const port = 3000;

const server = createServer((req, res) => {
  res.statusCode = 200;
  res.setHeader('Content-Type', 'text/plain');
  res.end('Hello World');
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});

```

<!--s-->
### flake.nix
```nix 
{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  in
  {
    packages.${system}.default = pkgs.writeShellScriptBin "run" ''
          nix develop -c -- node server.js
    '';
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        nodejs
        self.packages.${system}.default
      ];
    };

  };
}

```

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
### bash.sh
```sh 
$ nix build 
$ cat result/bin/run          
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
nix develop -c -- node server.js
$ nix run     
Server running at http://127.0.0.1:3000/

```

note:
- nix build
- similar to nix develop it will search for the flake
- and will evualate and build the default package in it
- this process run in "closed" nix building enviorment 
- which allows fetching of source code,configuring, patching, building
- this build process must end by creating a folder or a file in nix store
- when done by default it will create for us a result symlink to this file or folder in the current folder
---

<!--s-->
### flake.nix
```nix 
{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  in
  {
    packages.${system} = {
      start-server = pkgs.writeShellScript "run-server" ''
          node server.js
      '';
      # nix run should still start the service
      default = pkgs.writeShellScriptBin "run" ''
          nix develop -c -- ${self.packages.${system}.start-server}
      '';
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        nodejs
        # this wont work anymore becuase it is not a folder that contains bin with exectuable
        # self.packages.${system}.start-server
      ];
    };

  };
}

```

<!--v-->
### bash.sh
```sh 
$ nix build .\#start-server
$ cat ./result               
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
node server.js
$ nix run .\#start-server
error: unable to execute '/nix/store/2iplb5a63gn7m52ygljrwylp2jnjzwk2-run-server/bin/run-server': Not a directory

$ nix build                
$ cat result/bin/run 
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
nix develop -c -- /nix/store/2iplb5a63gn7m52ygljrwylp2jnjzwk2-run-server

$ nix run
Server running at http://127.0.0.1:3000/
$ ./result/bin/run  
Server running at http://127.0.0.1:3000/


```


<!--s-->
### commands.nix
```nix 
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

```

<!--v-->
### flake.nix
```nix 
{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  commands = import ./commands.nix { inherit pkgs; };
  in
  {
    packages.${system}= {
      start-server = commands.start-server;
      default = commands.default;
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        nodejs
        # we would like commands here
      ];
    };
  };
}

```

<!--v-->
### bash.sh
```sh 
$ nix build
$ cat ./result
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
nix develop -c -- /nix/store/bxix9almm64lj7nsvqcq7dfqcz6s8c80-start-server

$ nix run
error: unable to execute '/nix/store/2x80zxwggxvzgzffy395kp8bqw5flp87-default/bin/default': Not a directory

```


<!--s-->
### commands.nix
```nix 
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
in pkgs.lib.mapAttrs (key: command: pkgs.writeShellScriptBin name command) commands

```

<!--v-->
### flake.nix
```nix 
{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  commands = import ./commands.nix { inherit pkgs; };
  in
  {
    packages.${system}= {
      start-server = commands.start-server;
      default = commands.default;
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        nodejs
        # it will be nice to remove this repetition
        commands.start-server
        commands.default
      ];
    };
  };
}

```

<!--v-->
### bash.sh
```sh 
$ nix build
$ cat ./result/bin/default   
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
/nix/store/2x80zxwggxvzgzffy395kp8bqw5flp87-default

$ cat /nix/store/2x80zxwggxvzgzffy395kp8bqw5flp87-default
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
nix develop -c -- /nix/store/bxix9almm64lj7nsvqcq7dfqcz6s8c80-start-server

$ nix run             
Server running at http://127.0.0.1:3000/

```


<!--s-->
### commands.nix
```nix 
{ pkgs ? import <nixpkgs> {}
}:
let commands = pkgs.lib.fix (self: pkgs.lib.mapAttrs pkgs.writeShellScript
{
    start-server = ''
        node server.js
    '';
    default = ''
        nix develop -c -- ${self.start-server}
    '';
});
in pkgs.symlinkJoin rec {
  name = "default";
  passthru.bin = pkgs.lib.mapAttrs pkgs.writeShellScriptBin commands;
  paths = pkgs.lib.attrValues passthru.bin;
}

```

<!--v-->
### flake.nix
```nix 
{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  in
  {
    packages.${system} = {
      default = import ./commands.nix { inherit pkgs; };
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        nodejs
        self.packages.${system}.default.bin.start-server
      ];
    };
  };
}

```

<!--v-->
### bash.sh
```sh 
$ nix build
$ cat ./result/bin/default   
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
/nix/store/2x80zxwggxvzgzffy395kp8bqw5flp87-default

$ cat /nix/store/2x80zxwggxvzgzffy395kp8bqw5flp87-default
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
nix develop -c -- /nix/store/bxix9almm64lj7nsvqcq7dfqcz6s8c80-start-server

$ nix run             
Server running at http://127.0.0.1:3000/

```


<!--s-->
### commands.nix
```nix 
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

```

<!--v-->
### bash.sh
```sh 
$ nix repl '<nixpkgs>'
nix-repl> runCommand                              
«lambda @ /nix/store/fjv2fjy30lcf0g4rcyqrky7mxl5zy7yj-4fvp92iazfdj73g03wvnk8451mihhai5-source/pkgs/build-support/trivial-builders/default.nix:14:16»
  # See https://nixos.org/manual/nixpkgs/unstable/#trivial-builder-runCommand

```

<!--v-->
### flake.nix
```nix 
{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  commands = import ./commands.nix { inherit pkgs; };
  in
  {
    packages.${system}.default = commands;
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        nodejs
        commands
      ];
      shellHook = commands.set.welcome;
    };
  };
}

```


<!--s-->
# Live Coding
![live-coding](./images/live-coding.jpeg) <!-- .element width="90%" -->
<!--s-->
# Q & A
