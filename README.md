### What the Program does ###
nettool is a small toolkit with different tools.

At the moment only `subcalc` is available as a tool. It takes the IP in CIDR as Argument.
The tool takes an IPv4 address in CIDR format and computes, mask, first host, last host, broadcast, net address and total hosts.

### How to Use ###
Start the program via `cargo run -- [TOOL] [ARGUMENT]`

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

| Call                                 | Behaviour?                                 | Exit-Code | Channel |
| ------------------------------------ | ------------------------------------------ | --------- | ------- |
| `nettool` (empty)                    | print available tools                      | 2         | err out |
| `nettool subcalc 10.0.5.77/22`       | normal execution                           | 0         | std out |
| `nettool subcalc` (no Argument)      | "Tool missing arguments" & exit            | 2         | err out |
| `nettool subcalc murks`              | "Wrong input"             & exit           | 2         | err out |
| `nettool quatsch`                    | "Unknown Tool" print available tools & exit| 2         | err out |
| `nettool subcalc 10.0.5.77/22 extra` | "Too many arguments" & exit                | 2         | err out |


| Call                                  | Behaviour?                                 | Exit-Code | Channel |
| ------------------------------------  | ------------------------------------------ | --------- | ------- |
| `nettool maskcalc 255.255.252.0`      | normal execution outputs 22                | 0         | std out |
| `nettool maskcalc /22`                | normal execution outputs 255.255.252.0     | 0         | std out |
| `nettool maskcalc 22`                 | normal execution outputs 255.255.252.0     | 0         | std out |
| `nettool maskcalc` (no Argument)      | "Tool missing arguments" & exit            | 2         | err out |
| `nettool maskcalc murks`              | "Wrong input"             & exit           | 2         | err out |
| `nettool maskcalc 255.255.252.0 extra`| "Too many arguments" & exit                | 2         | err out |
| `nettool maskcalc 255.255.0.252`      | "Invalid Mask" & prompt & exit             | 2         | err out |
