phext shell
-----------
* a shell that is phext-aware
* keeps track of your current scroll by coordinate
* allows programs to pass hierarchical information between processes

Commands
--------
* vex: load a phext into memory
* cs: change scroll
* help: display online help

Overview
--------
This interactive shell is a swiss-army knife designed to make working with phexts simple and fun. The shell displays your current coordinate next to the prompt. Input and output for non-phext-aware programs is collected on the current scroll.

session example
---------------
1.1.1/1.1.1/1.1.1> hello-phext<LB>

Result: All output from the `hello-phext` process will be collected on the scroll starting at 2.1.1/1.1.1/1.1.1.
No additional programs can be started from this node, but we can change our current scroll with the `cs` command.

2.1.1/1.1.1/1.1.1> ls
ERROR: `hello-phext` is currently running. Switch to another scroll context to run a new program.

2.1.1/1.1.1/1.1.1> cs 1.1.1/1.1.1/1.1.2

Result: The user's I/O mount point will be adjusted to the given coordinate, which is not generating any output currently.

1.1.1/1.1.1/1.1.2> cs 1.1.1/1.1.1/1.1.1

Result: The user's I/O mount point will return to the root node, which is also not producing any output currently.