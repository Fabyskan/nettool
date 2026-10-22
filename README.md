### What the Program does ###
The program takes an IPv4 address in CIDR format and computes, mask, first host, last host, broadcast, net address and total hosts.

### How to Use ###
Start the program via `cargo run -- [TOOL] [ARGUMENT]`

At the moment only `subcalc` is available as a tool. It takes the IP in CIDR as Argument.

For example: `cargo run -- subcalc 10.0.5.77/22`
Output:
```
Prefix: 22
Netaddress: 10.0.4.0
Subnet Mask: 255.255.252.0
First Host: 10.0.4.1
Last Host: 10.0.7.254
Broadcast: 10.0.7.255
Total Hosts: 1022
```
Wrong input will be handled internally and the program will stop.
Exit Code 2 on ErrOut for Wrong Input 

### Tests ###

Tests are executed with `cargo test` 

### Error Codes ###


| Aufruf                               | Verhalten?                                 | Exit-Code | Kanal   |
| ------------------------------------ | ------------------------------------------ | --------- | ------- |
| `nettool` (ohne alles)               | Toolauswahl printen                        | 1         | std out |
| `nettool subcalc 10.0.5.77/22`       | ganz normales ausführen des Rechners       | 0         | std out |
| `nettool subcalc` (ohne Adresse)     | "Too few arguments" Meldung und abbruch    | 2         | err out |
| `nettool subcalc murks`              | Wrong input Meldung und abbruch            | 2         | err out |
| `nettool quatsch`                    | "Tool is not provided" Meldung und abbruch | 2         | err out |
| `nettool subcalc 10.0.5.77/22 extra` | "Too many arguments" Meldung und abbruch   | 2         | err out |