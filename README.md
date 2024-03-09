# Fast Cli tool for Keepass.info

Keepass is a great tool that I use day to day, but sometimes I don't want to open my whole vault just to get one password.
So this tool will try to be fast and secure to give you your passwords !

# Usage
The config (database path & key) can be set using a cli argument, so I recommend setting up an alias like:
`alias fcpass=fcpass /home/me/my_passwords.kdbx`
You can set the password (not recommended for obvious reasons):
`alias fcpass=fcpass /home/me/my_passwords.kdbx 1234`
If you don't set the password via cli args, it will be asked in an input 
## Commands
### Read
Let's say we have a db `Example.kdbx` with the password 1234 (in the project's root)
We have an entry named something like youtube, then we can find it using:
```sh
fcpass ./Example.kdbx 1234 read Youtube
```