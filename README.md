### Mosler is a shell for Hashicorp vault

It is in *very* early stages of development.

The goal is to be able to login once when starting mosler, and then run commands which perform 

operations using the vault HTTP api. The auth token will be saved for the login session and will be

used to communicate with the vault server.


Right now only the ls-policies command works. More query type commands will be added soon.


##### Dependencies

* rustyline (for the repl)
* reqwest (for sending requests over to the vault server)

