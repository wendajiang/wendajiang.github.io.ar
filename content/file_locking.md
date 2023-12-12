---
title: file locking
description: ''
template: blog/page.html
date: 2023-12-12 13:21:53
updated: 2023-12-12 13:21:53
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: []
extra:
  mermaid: true
  usemathjax: true
  lead: ''

# mermaid example: 
# {% mermaid() %}
#     mermaid program
# {% end %}
---

# overview
File locking is a mutual-exclusion mechanism to ensure a file can be read/written by muliple processes in a safe way.

# The interceding update problem
The interceding update is a **typical race condition** problem in a concurrent system. Let's see an example to understand the problem better.

Let's say we have a *balance.dat* file storing the balance of an account, and it has an initial value of "100". Our concurrent system has two processes to update the balance value:

1. Process A: reads the current value, substracts 20 and saves the result back to the file.
2. Process B: reads the current value, adds 80 and writes the result back into the file.

Obviously, after the execution of two processes, we are expects the file has value: 100 - 20 + 80 = 160.

However, an interdecing update problem may occur in this problem:

1. Process A reads the file's current value(100), and prepares to do further calculation.
2. Process B now reads the same file and gets the current balance(100).
3. Process A calculates 100 - 20 and saves the result 80 back to the file
4. Process B doesn't know the balance has been updated since its last read. So it will still use the stale value 100 to calculate 100 + 80 and write the result 180 to the file.

As a result, we have 180 in the *balance.dat* file instead of the expected value of 160.

# File locking in Linux
File locking is a mechanism to restrict access to a file among muliple processes. It allows only one process to access the file in a specific time, thus avoiding the interceding update problem.

Linux supports two kinds of the file locks: advisory locks and mandatory locks.

## Advisory lock
Advisory locking is not an enforced locking scheme. It will work only if the partipating processes are cooperating by explicityly acquiring locks. Otherwise, advisory locks will be ignored if a process is not aware of locks at all.

System call *flock* is an advisory lock.

## Mandaroty locking
Before we start looking at mandaroty file locking, we should keep in mind that [Linx implementation of mandaroty locking is unreliable](https://www.kernel.org/doc/Documentation/filesystems/mandatory-locking.txt).

Unlike advisory locking, mandatory locking doesn't require any cooperation between the paricipating processes. Once a mandatory lock is avrivated on a file, the operating system prevents other processes from reading or writing the file.

To enable mandatory file locking in Linux, two requirements must be satisfield.
1. we must mount the file system with the mand option (mount -o mand FILESYSTEM MOUNT_POINT)
2. we must turn on the set-group-ID bit and turn off the group-execute bit for the files we are about to lock (chmod g+s,g-x FILE)

# Inspect all locks in a system
## the lslocks command
We can use this command to see all the currently locked files in the system.
## /proc/locks
It is a file in the [procfs](https://en.wikipedia.org/wiki/Procfs) virtual file system. The file holds all current file locks, the *lslocks* command relies on this file to generate the list, too.

# Introduction to flock command
*flock* command is also provided by the util-linux package.
```bash
flock FILE_TO_LOCK COMMAND
```

## exmaple
```bash
#!/bin/bash
file="balance.dat"
value=$(cat $file)
echo "Read current balance:$value"

#sleep 10 seconds to simulate business calculation
progress=10
while [[ $progress -lt 101 ]]; do
	echo -n -e "\033[77DCalculating new balance..$progress%"
	sleep 1
	progress=$((10+progress))
done
echo ""

value=$((value+$1))
echo "Write new balance ($value) back to $file." 
echo $value > "$file"
echo "Done."
```

We create a simple shell script a.sh to simulate process A:
```bash
#!/bin/bash
#-----------------------------------------
# process A: lock the file and subtract 20 
# from the current balance
#-----------------------------------------
flock --verbose balance.dat ./update_balance.sh '-20'
```
Now let's start process A to test:
```bash
$ ./a.sh 
flock: getting lock took 0.000002 seconds
flock: executing ./update_balance.sh
Read current balance:100
Calculating new balance..100%
Write new balance (80) back to balance.dat.
Done.
```
Through the output, we can see the *flock* command first acquired a lock on the file *balance.dat*, then the *update_balance.sh* script read and updated the file.

During its run, we can check the lock information via the *lslocks* command:
```bash
$ lslocks | grep "balance"
flock  825712 FLOCK 4B WRITE 0 0 0 /tmp/test/balance.dat
```

The output shows that the *flock* command is holding a WRITE lock on the entire file /tmp/test/balance.dat


# reference

- [File locking.hpp](https://github.com/jeandiogo/locker)
- man 2 flock
- man 2 readlink
- [file hard link](https://en.wikipedia.org/wiki/Hard_link)