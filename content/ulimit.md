+++
template = "blog/page.html"
date = "2021-04-02 14:39:37"
title = "Linux ulimit"
[taxonomies]
tags = ["linux", "tool"]

[extra]
mermaid = true
usemathjax = true
+++
<!--
mermaid example:
<div class="mermaid">
    mermaid program
</div>
-->

命  令：ulimit

功  能：控制shell程序的资源

语　　法：`ulimit [-aHS][-c ][-d <数据节区大小>][-f <文件大 小>][-m <内存大小>][-n <文件数目>][-p <缓冲区大小>][-s <堆栈大小>][-t <CPU时间>][-u <程序数目>][-v <虚拟内存大小>]`

  补充说明：ulimit为shell内建指令，可用来控制shell执行程序的资源。 

 

  参　　数： 

-H 设置硬件资源限制,是管理员所设下的限制.

-S 设置软件资源限制,是管理员所设下的限制.

-a 显示当前所有的资源限制.

-u 进程数目:用户最多可启动的进程数目.

-c size:设置core文件的最大值.单位:blocks

-d size:设置程序数据段的最大值.单位:kbytes

-f size:设置shell创建文件的最大值.单位:blocks

-l size:设置在内存中锁定进程的最大值.单位:kbytes

-m size:设置可以使用的常驻内存的最大值.单位:kbytes

-n size:设置内核可以同时打开的文件描述符的最大值.单位:n

-p size:设置管道缓冲区的最大值.单位:kbytes

-s size:设置堆栈的最大值.单位:kbytes

-t size:设置CPU使用时间的最大上限.单位:seconds

-v size:设置虚拟内存的最大值.单位:kbytes

 

 

### 系统调优

 

  如前所述， ulimit -a 用来显示当前的各种用户进程限制。

  Linux对于每个用户，系统限制其最大进程数。为提高性能，可以根据设备资源情况，

  设置各linux 用户的最大进程数，下面我把某linux用户的最大进程数设为10000个： 

 

   ulimit -u 10000 

   对于需要做许多 socket 连接并使它们处于打开状态的 Java 应用程序而言，

   最好通过使用 ulimit -n xx 修改每个进程可打开的文件数，缺省值是 1024。 

   ulimit -n 4096 

   将每个进程可以打开的文件数目加大到4096，缺省为1024   

   其他建议设置成无限制（unlimited）的一些重要设置是： 

   数据段长度：ulimit -d unlimited 

   最大内存大小：ulimit -m unlimited 

   堆栈大小：ulimit -s unlimited 

   CPU 时间：ulimit -t unlimited 

   虚拟内存：ulimit -v unlimited 

　　

   暂时地，适用于通过 ulimit 命令登录 shell 会话期间。

   永久地，通过将一个相应的 ulimit 语句添加到由登录 shell 读取的文件中， 即特定于 shell 的用户资源文件，如： 

 

#### 1) 解除 Linux 系统的最大进程数和最大文件打开数限制：

​    vi /etc/security/limits.conf

```
​    \# 添加如下的行

​    \* soft noproc 11000

​    \* hard noproc 11000

​    \* soft nofile 4100

​    \* hard nofile 4100 
```
​    说明：* 代表针对所有用户

​      noproc 是代表最大进程数

​      nofile 是代表最大文件打开数 

 

#### 2) 修改所有 linux 用户的环境变量文件：

vi /etc/profile 

ulimit -u 10000

ulimit -n 4096

ulimit -d unlimited 

ulimit -m unlimited 

ulimit -s unlimited 

ulimit -t unlimited 

ulimit -v unlimited 

 

/**************************************

 

有时候在程序里面需要打开多个文件，进行分析，系统一般默认数量是1024，（用ulimit -a可以看到）对于正常使用是够了，但是对于程序来讲，就太少了。

修改2个文件。

1) /etc/security/limits.conf

vi /etc/security/limits.conf

加上：

\* soft nofile 8192

\* hard nofile 20480

2) /etc/pam.d/login

session required /lib/security/pam_limits.so

**********

另外确保/etc/pam.d/system-auth文件有下面内容

session required /lib/security/$ISA/pam_limits.so

这一行确保系统会执行这个限制。

***********

3) 一般用户的.bash_profile

\#ulimit -n 1024

重新登陆ok

 

#### 3) /proc目录：

1）/proc目录里面包括很多系统当前状态的参数，例如：引用

/proc/sys/fs/file-max

/proc/sys/fs/inode-max

 

 

是对整个系统的限制，并不是针对用户的；

2）proc目录中的值可以进行动态的设置，若希望永久生效，可以修改/etc/sysctl.conf文件，并使用下面的命令确认：

\# sysctl -p

ulimit 用于限制 shell 启动进程所占用的资源，支持以下各种类型的限制：

  所创建的内核文件的大小、

  进程数据块的大小、

  Shell 进程创建文件的大小、

  内存锁住的大小、

  常驻内存集的大小、

  打开文件描述符的数量、

  分配堆栈的最大大小、

  CPU 时间、

  单个用户的最大线程数、

  Shell 进程所能使用的最大虚拟内存。同时，它支持硬资源和软资源的限制。

作为临时限制，ulimit 可以作用于通过使用其命令登录的 shell 会话，在会话终止时便结束限制，并不影响于其他 shell 会话。而对于长期的固定限制，ulimit 命令语句又可以被添加到由登录 shell 读取的文件中，作用于特定的 shell 用户。

 

2、使用ulimit
ulimit 通过一些参数选项来管理不同种类的系统资源。

ulimit 命令的格式为：ulimit [options] [limit]

 

主要关注两个：

1）open files：– 用户可以打开文件的最大数目

对应ulimit 的命令ulimit -n，可以使用ulimit -n 临时设置。

对应/etc/security/limits.conf的资源限制类型是：nofile

\* soft nofile 4096  

 \* hard nofile 4096

2）max user processes – 用户可以开启进程/线程的最大数目

对应ulimit 的命令ulimit  -u 临时修改max user processes的值：ulimit  -u  8192。

对应/etc/security/limits.conf的资源限制类型是： noproc

\*      soft   nproc   8192

 

具体的 options 含义以及简单示例可以参考以下表格。
ulimit 参数说明

选项 含义-a 显示当前系统所有的limit资源信息。 -H 设置硬资源限制，一旦设置不能增加。例如：ulimit – Hs 64；限制硬资源，线程栈大小为 64K。-S 设置软资源限制，设置后可以增加，但是不能超过硬资源设置。例如：ulimit – Sn 32；限制软资源，32 个文件描述符。-c 最大的core文件的大小，以 blocks 为单位。例如：ulimit – c unlimited； 对生成的 core 文件的大小不进行限制。-f 进程可以创建文件的最大值，以blocks 为单位.例如：ulimit – f 2048；限制进程可以创建的最大文件大小为 2048 blocks。-d 进程最大的数据段的大小，以Kbytes 为单位。例如：ulimit -d unlimited；对进程的数据段大小不进行限制。-m 最大内存大小，以Kbytes为单位。例如：ulimit – m unlimited；对最大内存不进行限制。-n 可以打开的最大文件描述符的数量。例如：ulimit – n 128；限制最大可以使用 128 个文件描述符-s 线程栈大小，以Kbytes为单位。例如:ulimit – s 512；限制线程栈的大小为 512 Kbytes。-p 管道缓冲区的大小，以Kbytes 为单位。例如ulimit – p 512；限制管道缓冲区的大小为 512 Kbytes。-u 用户最大可用的进程数。例如 limit – u 65536；限制用户最多可以使用 65536个进程。-v 进程最大可用的虚拟内存，以Kbytes 为单位。ulimit – v 200000；限制最大可用的虚拟内存为 200000 Kbytes。-t 最大CPU占用时间，以秒为单位。ulimit – t unlimited；对最大的 CPU 占用时间不进行限制。-l 最大可加锁内存大小，以Kbytes 为单位。
1


我们可以通过以下几种方式来使用 ulimit：

## 一、在用户的启动脚本中

​    如果用户使用的是 bash，就可以在用户的目录下的 .bashrc 文件中，加入 ulimit – u 64，来限制用户最多可以使用 64 个进程。此外，可以在与 .bashrc 功能相当的启动脚本中加入 ulimt。

## 二、在应用程序的启动脚本中

如果用户要对某个应用程序 myapp 进行限制，可以写一个简单的脚本 startmyapp。

ulimit – s 512
myapp

以后只要通过脚本 startmyapp 来启动应用程序，就可以限制应用程序 myapp 的线程栈大小为 512K。

 

## 三、直接在控制台输入 

 ulimit – p 256 

限制管道的缓冲区为 256K。

## 四、修改所有 linux 用户的环境变量文件：

  vi /etc/profile

  ulimit -u 10000

  ulimit -n 4096

  ulimit -d unlimited

  ulimit -m unlimited

  ulimit -s unlimited

  ulimit -t unlimited

  ulimit -v unlimited

 保存后运行#source /etc/profile 使其生效

四、也可以针对单个用户的.bash_profile设置：

vi ~./.bash_profile

\#ulimit -n 1024
重新登陆ok

 

3、用户进程的有效范围
ulimit 作为对资源使用限制的一种工作，是有其作用范围的。那么，它限制的对象是单个用户，单个进程，还是整个系统呢？事实上，ulimit 限制的是当前 shell 进程以及其派生的子进程。举例来说，如果用户同时运行了两个 shell 终端进程，只在其中一个环境中执行了 ulimit – s 100，则该 shell 进程里创建文件的大小收到相应的限制，而同时另一个 shell终端包括其上运行的子程序都不会受其影响：

Shell 进程 1

ulimit –s 100
cat testFile > newFile
File size limit exceeded

Shell 进程 2

cat testFile > newFile
ls –s newFile
323669 newFile

 

针对用户永久生效：

那么，是否有针对某个具体用户的资源加以限制的方法呢？答案是有的，方法是通过修改系统的 /etc/security/limits.conf配置文件。该文件不仅能限制指定用户的资源使用，还能限制指定组的资源使用。该文件的每一行都是对限定的一个描述。

limits.conf的格式如下：

<domain>          <type>    <item>   <value> 

username|@groupname    type    resource      limit

domain：username|@groupname：设置需要被限制的用户名，组名前面加@和用户名区别。也可以用通配符*来做所有用户的限制。

type：有 soft，hard 和 -，soft 指的是当前系统生效的设置值。hard 表明系统中所能设定的最大值。soft 的最大值不能超过hard的值。用 – 就表明同时设置了 soft 和 hard 的值。

resource：
  core – 限制内核文件的大小
  date – 最大数据大小
  fsize – 最大文件大小
  memlock – 最大锁定内存地址空间
  nofile – 打开文件的最大数目
  rss – 最大持久设置大小
  stack – 最大栈大小
  cpu – 以分钟为单位的最多 CPU 时间
  noproc – 进程的最大数目（系统的最大进程数）
  as – 地址空间限制
  maxlogins – 此用户允许登录的最大数目

  要使 limits.conf 文件配置生效，必须要确保 pam_limits.so 文件被加入到启动文件中。

  查看 /etc/pam.d/login 文件中有：
  session required /lib/security/pam_limits.so

 

例如：解除 Linux 系统的最大进程数和最大文件打开数限制：  

​    vi /etc/security/limits.conf  

​    \# 添加如下的行  

​    \* soft noproc 20000 #软连接  

​    \* hard noproc 20000  #硬连接  

​    \* soft nofile 4096  

​    \* hard nofile 4096  

​    说明：* 代表针对所有用户，noproc 是代表最大进程数，nofile 是代表最大文件打开数

 

需要注意一点：/etc/security/limits.d下也有noproc最大进参数的限制：

即 /etc/security/limits.d/下的文件覆盖了/etc/security/limits.conf设置的值 

这个是官网答疑：https://access.redhat.com/solutions/406663

\# /etc/security/limits.conf
\#This file sets the resource limits for the users logged in via PAM.
\#It does not affect resource limits of the system services.
\#Also note that configuration files in /etc/security/limits.d directory,
\#That means for example that setting a limit for wildcard domain here

[root@tr10-46-65-29 ~]# cat /etc/security/limits.d/20-nproc.conf 
\# Default limit for number of user's processes to prevent
\# accidental fork bombs.
\# See rhbz #432903 for reasoning.

\*      soft   nproc   8192
root    soft   nproc   unlimited

现在已经可以对进程和用户分别做资源限制了，看似已经足够了，其实不然。很多应用需要对整个系统的资源使用做一个总的限制，这时候我们需要修改 /proc 下的配置文件。/proc 目录下包含了很多系统当前状态的参数，例如 /proc/sys/kernel/pid_max，/proc/sys/net/ipv4/ip_local_port_range 等等，从文件的名字大致可以猜出所限制的资源种类。

注意：

通过读取/proc/sys/fs/file-nr可以看到当前使用的文件描述符总数。另外，对于文件描述符的配置，需要注意以下几点：


所有进程打开的文件描述符数不能超过/proc/sys/fs/file-max

 


单个进程打开的文件描述符数不能超过user limit中nofile的soft limit

 


nofile的soft limit不能超过其hard limit

 


nofile的hard limit不能超过/proc/sys/fs/nr_open


4、用户进程的有效范围
 问题1：su切换用户时提示：Resource temporarily unavailable

或者通过进程跟踪 strace -p pid 看到:Resource temporarily unavailab

通过ulimit -a，得到结果：

core file size      (blocks, -c) 0
data seg size      (kbytes, -d) unlimited
scheduling priority       (-e) 0
file size        (blocks, -f) unlimited
pending signals         (-i) 63463
max locked memory    (kbytes, -l) 64
max memory size     (kbytes, -m) unlimited
open files            (-n) 65535
pipe size       (512 bytes, -p) 8
POSIX message queues   (bytes, -q) 819200
real-time priority        (-r) 0
stack size        (kbytes, -s) 8192
cpu time        (seconds, -t) unlimited
max user processes        (-u) 4096
virtual memory      (kbytes, -v) unlimited
file locks            (-x) unlimited

 

在上面这些参数中，通常我们关注得比较多:

open files: 一个进程可打开的最大文件数.

max user processes: 系统允许创建的最大进程数量.

通过 ps -efL|grep java |wc -l

\#24001

这个得到的线程数竟然是2万多，远远超过4096

我们可以使用 ulimit -u 20000 修改max user processes的值，但是只能在当前终端的这个session里面生效，重新登录后仍然是使用系统默认值。

正确的修改方式是修改/etc/security/limits.d/20-nproc.conf文件中的值。先看一下这个文件包含什么：

$ cat /etc/security/limits.d/90-nproc.conf # Default limit for number of user's processes to prevent# accidental fork bombs.# See rhbz #432903 for reasoning.*     soft  nproc  8192
我们只要修改上面文件中的8192这个值，即可。

 

问题2：linux 打开文件数 too many open files 解决方法
在运行某些命令或者 tomcat等服务器持续运行 一段时间后可能遇到  too many open files。

出现这句提示的原因是程序打开的文件/socket连接数量超过系统设定值。

java进程如果遇到java.net.SocketException: Too many open files，接着可能导致域名解析ava.net.UnknownHostException:

原因是用户进程无法打开系统文件了。

 

查看每个用户最大允许打开文件数量


ulimit -a

其中 open files (-n) 65535 表示每个用户最大允许打开的文件数量是65535 。 默认是1024。1024很容易不够用。

永久修改open files 方法
vim /etc/security/limits.conf  
在最后加入  
\* soft nofile 65535 
\* hard nofile 65535  

或者只加入

 \* - nofile 65535

最前的 * 表示所有用户，可根据需要设置某一用户，例如
fdipzone soft nofile 8192  
fdipzone hard nofile 8192  

注意"nofile"项有两个可能的限制措施。就是项下的hard和soft。 要使修改过得最大打开文件数生效，必须对这两种限制进行设定。 如果使用"-"字符设定, 则hard和soft设定会同时被设定。 
改完后注销一下就能生效。

通过 ulimit -n或者ulimit -a 查看系统的最大文件打开数已经生效了。但此时查看进程的最大文件打开数没有变，原因是这个值是在进程启动的时候设定的，要生效必须重启！

 

ok，那就重启吧，重启完毕，结果发现依然没变！这奇了怪了，后来经过好久的排查，最终确认问题是，该程序是通过 supervisord来管理的，也就是这进程都是 supervisord 的子进程，而 supervisord 的最大文件打开数还是老的配置，此时必须重启 supervisord 才可以。

当大家遇到limits修改不生效的时候，请查一下进程是否只是子进程，如果是，那就要把父进程也一并重启才可以。

### 1. limits是一个进程的资源，会被子进程继承

 

### 2. soft limit -S, hard limits -H

hard limits只能被root用户修改，启动的时候会加载配置/etc/security/limits.conf

soft limits可以被任何用户修改，但不能超过hard limits

 

### 3. 在linux下，每个进程的limit信息保存在/proc/PID/limits文件中(linux OS kenerl > 2.6.24)。低于2.6.24版本的kenerl需要手动统计 /proc/PID/fd下面有多个少个文件。

 

### 4. lsof -p pid显示所有的打开文件包括shared library

lsof 会统计一些duplicate的open file

 

### 5. system-wide fd

sysctl -a

vim /etc/sysctl.conf

 

### 6. max open file on the system

cat /proc/sys/fs/file-max

 

### 7. stat the openning file from the kenerl point of view

```bash
cat /proc/sys/fs/file-nr 
864     0       3274116
have 864 out of max 3274116 open files
```

### 8.利用lsof统计每个进程打开的文件数目
```
lsof -n |awk '{print $2}'|sort|uniq -c |sort -nr|more 
```

### 9. 设置普通用户下打开文件的最大值
 ulimit -n 4096
-bash: ulimit: open files: cannot modify limit: Operation not permitted
#### 9.1 在/etc/security/limits.conf中添加
* hard nofile 100000
* soft nofile 100000
#### 9.2 /etc/pam.d/login 添加
session required     /lib64/security/pam_limits.so
#### 9.3 重启 ssh2和rccron，这样只进程就自动继承了nofile
/etc/init.d/ssh2 restart
rccron restart
