$ nix build
$ cat ./result/bin/default   
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
/nix/store/2x80zxwggxvzgzffy395kp8bqw5flp87-default

$ cat /nix/store/2x80zxwggxvzgzffy395kp8bqw5flp87-default
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
nix develop -c -- /nix/store/bxix9almm64lj7nsvqcq7dfqcz6s8c80-start-server

$ nix run             
Server running at http://127.0.0.1:3000/
