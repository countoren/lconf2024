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

