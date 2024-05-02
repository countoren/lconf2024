$ nix build 
$ cat result/bin/run          
#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
nix develop -c -- node server.js
$ nix run     
Server running at http://127.0.0.1:3000/
