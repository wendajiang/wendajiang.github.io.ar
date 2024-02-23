---
title: How to check the port if used in unix/linux
description: ''
template: blog/page.html
date: 2024-02-23 19:25:01
updated: 2024-02-23 19:25:01
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["report", "linux", "tool"]
extra:
  mermaid: false
  usemathjax: true
  lead: ''
---

## [How to check if port is in use](https://www.cyberciti.biz/faq/unix-linux-check-if-port-is-in-use-command/)

To check the listening ports and applications on Linux:

1. Open a terminal application i.e. shell prompt.
2. Run any one of the following command on Linux to see open ports:
   ```shell
   $ sudo lsof -i -P -n | grep LISTEN
   $ sudo netstat -tulpn | grep LISTEN
   $ sudo ss -tulpn | grep LISTEN
   $ sudo lsof -i:22 ## see a specific port such as 22 ##
   $ sudo nmap -sTU -O IP-address-Here
   ```
3. For the latest version of Linux use the ss command. For example, `ss -tulwnp`

Let us see commands and its output in details.

## Option #1: lsof command

The syntax is:
```shell
$ sudo lsof -i -P -n
$ sudo lsof -i -P -n | grep LISTEN
$ doas lsof -i -P -n | grep LISTEN # OpenBSD #
```



## Option #2: netstat or ss command

You can check the listening ports and applications with netstat as follows.

### Linux netstat syntax

**Prerequisite**
By default, `netstat` command may not be installed on your system. Hence, use the [apk command](https://www.cyberciti.biz/faq/10-alpine-linux-apk-command-examples/) on Alpine Linux, dnf command/[yum command](https://www.cyberciti.biz/faq/rhel-centos-fedora-linux-yum-command-howto/) on RHEL & co, [apt command](https://www.cyberciti.biz/faq/ubuntu-lts-debian-linux-apt-command-examples/)/[apt-get command](https://www.cyberciti.biz/tips/linux-debian-package-management-cheat-sheet.html) on Debian, Ubuntu & co, zypper command on SUSE/OpenSUSE, pacman command on Arch Linux to install the `netstat`.

Run the netstat command along with [grep command](https://www.cyberciti.biz/faq/howto-use-grep-command-in-linux-unix/) to filter out port in LISTEN state:
```shell
$ netstat -tulpn | grep LISTEN
$ netstat -tulpn | more
```
OR filter out specific TCP port such as 443:
`$ netstat -tulpn | grep ':443'`
Where netstat command options are:


- **-t** : Select all TCP ports
- **-u** : Select all UDP ports
- **-l** : Show listening server sockets (open TCP and UDP ports in listing state)
- **-p** : Display PID/Program name for sockets. In other words, this option tells who opened the TCP or UDP port. For example, on my system, Nginx opened TCP port 80/443, so I will /usr/sbin/nginx or its PID.
- **-n** : Don’t resolve name (avoid dns lookup, this speed up the netstat on busy Linux/Unix servers)

The netstat command deprecated for some time on Linux. Therefore, you need to use the ss command as follows:
```shell
$ sudo ss -tulw
$ sudo ss -tulwn
$ sudo ss -tulwn | grep LISTEN
```
Where, ss command options are as follows:

- **-t** : Show only TCP sockets on Linux
- **-u** : Display only UDP sockets on Linux
- **-l** : Show listening sockets. For example, TCP port 22 is opened by SSHD server.
- **-p** : List process name that opened sockets
- **-n** : Don’t resolve service names i.e. don’t use DNS

Related: [Linux Find Out Which Process Is Listening Upon a Port](https://www.cyberciti.biz/faq/what-process-has-open-linux-port/)

### FreeBSD/macOS (OS X) netstat syntax

The syntax is as follows:
```shell
$ netstat -anp tcp | grep LISTEN
$ netstat -anp udp | grep LISTEN
```
You can use the sockstat command on macOS or [FreeBSD to display open TCP or UDP ports](https://www.cyberciti.biz/faq/freebsd-unix-find-the-process-pid-listening-on-a-certain-port-commands/) too. For example:
`{vivek@freebsd13-server:~}$ sudo sockstat -4 -6 -l`
Outputs from my [FreeBSD server version](https://www.cyberciti.biz/faq/how-to-find-out-freebsd-version-and-patch-level-number/) 13.xx:

```
USER     COMMAND    PID   FD PROTO  LOCAL ADDRESS         FOREIGN ADDRESS      
root     master     1723  13 tcp4   127.0.0.1:25          *:*
root     master     1723  14 tcp4   192.168.2.20:25       *:*
root     sshd       1627  3  tcp6   *:22                  *:*
root     sshd       1627  4  tcp4   *:22                  *:*
ntpd     ntpd       1615  20 udp6   *:123                 *:*
ntpd     ntpd       1615  21 udp4   *:123                 *:*
ntpd     ntpd       1615  22 udp4   192.168.2.20:123      *:*
ntpd     ntpd       1615  23 udp6   ::1:123               *:*
ntpd     ntpd       1615  24 udp6   fe80::1%lo0:123       *:*
ntpd     ntpd       1615  25 udp4   127.0.0.1:123         *:*
ntpd     ntpd       1615  26 udp4   172.16.0.5:123        *:*
root     syslogd    1085  6  udp6   *:514                 *:*
root     syslogd    1085  7  udp4   *:514                 *:*
?        ?          ?     ?  udp4   *:17890               *:*
?        ?          ?     ?  udp6   *:17890               *:*
```


## Option #3: nmap command

The syntax is:
```shell
$ sudo nmap -sT -O localhost# search for open port IP address 192.168.2.13
$ sudo nmap -sU -O 192.168.2.13 ##[ list open UDP ports ]
$ sudo nmap -sT -O 192.168.2.13 ##[ list open TCP ports ]
```

You can combine TCP/UDP scan in a single command:
`$ sudo nmap -sTU -O 192.168.2.13`



## Testing if a port is open from a bash script

One can use the “`/dev/tcp/{HostName}_OR_{IPAddrress}>/{port}`” syntax to check if a TCP port is open on a Linux or Unix machine when using Bash. In other words, the following is Bash specific feature. Let us see if TCP port 22 is open on localhost and 192.168.2.20:
```shell
$ (echo >/dev/tcp/localhost/23) &>/dev/null && echo "open" || echo "close"
$ (echo >/dev/tcp/192.168.2.20/22) &>/dev/null && echo "open" || echo "close"
```
Now we can build some logic as follows:

```bash
#!/bin/bash
dest_box="aws-prod-server-42"
echo "Testing the ssh connectivity ... "
if ! (echo >/dev/tcp/$dest_box/22) &>/dev/null
then
    echo "$0 cannot connect to the $dest_box. Check your vpn connectivity."
else
    echo "Running the ansible playboook ..."
    ansible-playbook -i hosts --ask-vault-pass --extra-vars '@cluster.data.yml' main.yaml
fi
```

### What if I’m not using Bash…

Try the nc command as follows:
```shell
$ nc -w {timeout} -zv {server_IP_hostname} {tcp_port} &>/dev/null && echo "Open" || echo "Close"
$ nc -w 5 -zv 192.168.2.20 23 &>/dev/null && echo "TCP/23 Open" || echo "TCP/23 Close"
```
The updated Bash script:

```shell
#!/bin/bash
dest_box="aws-prod-server-42"
timeout="5" # timeouts in seconds
echo "Testing the ssh connectivity in $timeout seconds ... "
# make sure 'nc' is installed, else die ..
if ! type -a nc &>/dev/null
then
    echo "$0 - nc command not found. Please install nc and run the script again."
    exit 1
fi
if !  nc -w "$timeout" -zv "${dest_box}" 22  &>/dev/null
then
    echo "$0 cannot connect to the $dest_box. Check your vpn connectivity."
    exit 1
else
    echo "Running the ansible playboook ..."
    ansible-playbook -i hosts --ask-vault-pass --extra-vars '@cluster.data.yml' main.yaml
fi
```

## Using Perl to check if a TCP port is open in Linux or Unix

Here is a Perl script to check if TCP port 22 for OpenSSH is open with a 5-second timeout using [IO::Socket::INET](https://perldoc.perl.org/IO::Socket::INET):

```perl
#!/usr/bin/perl -w 
use IO::Socket::INET;
 
# Set server name and port here
$my_server="192.168.2.20";
$my_server_tcp_port="22";
 
# make a new object
my $server_test = IO::Socket::INET->new(
  PeerAddr => "$my_server",
  PeerPort => "$my_server_tcp_port",
  Proto => 'tcp',
  Timeout => 5
);
 
# test it and die or continue as per your needs
if ($server_test) {
  print "TCP port $my_server_tcp_port is open for the $my_server.\n";
  print "Now doing something ...\n";
  close $server_test;
} 
else {
  print "TCP port $my_server_tcp_port is closed or timed out for the $my_server.\n";
}
```

## Python example to check if a TCP port is open in Linux or Unix

Try thise simple code that uses [low level socket](https://docs.python.org/3/library/socket.html) networking feature. For example:

```python
#!/usr/bin/python3
# Tested on Python 3.6.xx and 3.8.xx only (updated from Python 2.x)
import socket
 
# Create a new function 
def check_server_tcp_port(my_host_ip_name, my_tcp_port, timeout=5):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.settimeout(timeout)
    try:
        s.connect((my_host_ip_name, my_tcp_port))
        print(f"TCP port {my_tcp_port} is open for the {my_host_ip_name}.")
        s.close()
        return True
    except socket.timeout:
        print(f"TCP port {my_tcp_port} is closed or timed out for the {my_host_ip_name}.")
        return False
 
# Test it 
check_server_tcp_port("localhost", 22)
check_server_tcp_port("192.168.2.20", 22)
```

## Conclusion

This page explained command to determining if a port is in use on Linux or Unix-like server. For more information see the [nmap command](https://www.cyberciti.biz/networking/nmap-command-examples-tutorials/) and lsof command page [online here](https://github.com/lsof-org/lsof) or by typing the [man command](https://bash.cyberciti.biz/guide/Man_command) as follows:
`$ man lsof$ man ss$ man netstat$ man nmap$ man 5 services$ man nc`

## See also

- [How to ping and test for a specific port from Linux or Unix command line](https://www.cyberciti.biz/faq/ping-test-a-specific-port-of-machine-ip-address-using-linux-unix/)
- [30 Handy Bash Shell Aliases For Linux / Unix / MacOS](https://www.cyberciti.biz/tips/bash-aliases-mac-centos-linux-unix.html)
- [Linux and Unix Port Scanning With netcat {nc} Command](https://www.cyberciti.biz/faq/linux-port-scanning/)
- [Nmap Command Examples For Linux Users / Admins](https://www.cyberciti.biz/security/nmap-command-examples-tutorials/)
