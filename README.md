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



### Tests ###

Tests are executed with `cargo test` 

### Error Codes ###
Wrong input will be handled internally and the program will stop.

| Aufruf                               | Verhalten?                                 | Exit-Code | Kanal   |
| ------------------------------------ | ------------------------------------------ | --------- | ------- |
| `nettool` (ohne alles)               | Toolauswahl printen                        | 2         | err out |
| `nettool subcalc 10.0.5.77/22`       | ganz normales ausführen des Rechners       | 0         | std out |
| `nettool subcalc` (ohne Adresse)     | "Too missing arguments" Meldung und abbruch| 2         | err out |
| `nettool subcalc murks`              | Wrong input Meldung und abbruch            | 2         | err out |
| `nettool quatsch`                    | "Unknown Tool" Toolauswhahl print & abbruch| 2         | err out |
| `nettool subcalc 10.0.5.77/22 extra` | "Too many arguments" Meldung und abbruch   | 2         | err out |

```
cargo run --  ; echo $?
cargo run -- subcalc 10.0.5.77/22; echo $?
cargo run -- subcalc ; echo $?
cargo run -- subcalc murks ; echo $?
cargo run -- quatsch ; echo $?
cargo run -- subcalc 10.0.5.77/22 extra; echo $?
```