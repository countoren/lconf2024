# Notes

## Problem 

Without nix it is a lot harder to leverage different tools from different enviornoments.

## Solution

nix allows you to combine pre made tools such as executables and langueage specific libraries 
either your own or 3rd party.
which gives a lot more options to solve your problems
and in this talk I will try to show you how you can do it effienctly leveraging nix shell scripts 


## 1
let start with a simple nix script that give you an empty development shell:


## example

<!-- You could ask yourself, should I use curl instead of writing a rest request implementation in a specific langague? -->
as simple real world example that I had to solve was while implementing a CI pipeline I had to make Jira be updated when I finish a pipeline build
I had a couple of choices but I pick to just implement it with a simple curl request that will run at the end of the build and update all the Jira tickets related to the release in hand.

This basicly the "vertical archticture" taking language and ecosystem and cli tools as the layers 

---

this step-9 expand our commands from the step before
to a gap between multiple eco systems: Nodejs, FSharp and Rust.
we added model.fsx that will define some simple types to shared across
our NodeJS server and our Rust CLI tool we added commands to consume the
server with curl and pipe it to our rust cli which in turn will parse
the json with the common types transpiled from model.fsx

we will can git ignore fable_module for the JS eco system becuase it
does not need to create a "nix flake package", we wont be able to ignore
for now the fable_modules in the rust project due to the fact they are 
"counted" as part of the source code to build the rust project.
there are way to solve it but for sake of simplicity I am going to leave
it for now.
