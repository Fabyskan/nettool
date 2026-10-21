### What the Program does ###
The program takes an IPv4 address in CIDR format and computes, mask, first host, last host, broadcast, net address and total hosts.

### How to Use ###
Start the program via `cargo run`
When prompted with
`Please input your IP in CIDR.`
Enter your IP in CIDR
For example: `10.0.5.77/22`
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
Wrong input will be handled internally and the user will be prompted to make another try.

### Tests ###

Tests are executed with `cargo test` 