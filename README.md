### Mosler is a shell for Hashicorp vault

It is in *very* early stages of development.

The goal is to be able to login once when starting mosler, and then run commands which perform operations using the vault HTTP api. 

The auth token will be cached for the login session and will be used to communicate with the vault server.

<br>

Right now only the ls-policies command works. More query commands are being added.

<br>

##### Dependencies

* rustyline (for the repl, tab completion, history etc)
* reqwest (for sending requests to the vault server over HTTP)

