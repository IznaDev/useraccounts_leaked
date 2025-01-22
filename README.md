# useraccounts_leaked
A Rust project to check if your account are leaked.

In this code I offer to check if your account was leaked thanks to the API  "Have I been pwned?". To ensure the safety of your data, this code hash your login and your password before send them to the API as a long hexadecimal hashing representing your password. 

You can create a file with lot of "login:password" with this shape. one "login:password" per line like the exemple bellow :

clibbord:d8pJ
shemia:hJxBuMUyp?
dynila:iyvQ?
...

We make a simple  get request to the API to retrieve the hashing suffix of  leaked passwords  with the crate reqwest. 

In the next step we will do asuynchronous request with tokio. 

The code is made up from 3 modules: account, error, hibp and a main.

A very important thing is not here... The tests. I will do all needed tests in a very close futur.

to run you have to build before: cargo build

to run with a file : target/debug/useraccount_leaked group --file [FILEPATH]

A file is present in the docs folder to make a test.

clone, build, run and enjoy. 

Rust is so coool ! I've converted to crab and I invite you to do the same ;p !!