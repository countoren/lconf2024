{
  outputs = { self, nixpkgs }:
  let system = "x86_64-linux";
  pkgs = import nixpkgs { inherit system;};
  in
  {
    packages.${system} = {
      /*
$ nix build
$ cat ./result/bin/default   
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
/nix/store/2x80zxwggxvzgzffy395kp8bqw5flp87-default

$ cat /nix/store/2x80zxwggxvzgzffy395kp8bqw5flp87-default
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
nix develop -c -- /nix/store/bxix9almm64lj7nsvqcq7dfqcz6s8c80-start-server

$ nix run             
Server running at http://127.0.0.1:3000/
      */
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
