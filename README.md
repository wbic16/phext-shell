phext shell
-----------
* a shell that is phext-aware
* keeps track of your current scroll by coordinate
* allows programs to pass hierarchical information between processes

Commands
--------
* af: appends the contents of the specified file to the current coordinate
* cs: Change scroll
* ds: Display scroll
* lp: Loads a phext into memory
* os: overwrites the current scroll with the specified text
* rp: resets the current phext
* rs: resets the current scroll
* sp: saves the current phext to disk
* help: display online help

Overview
--------
This interactive shell is a swiss-army knife designed to make working with phexts simple and fun. The shell displays your current coordinate next to the prompt. Input and output for non-phext-aware programs is collected on the current scroll.

Hierarchical History
--------------------
Upon terminating your phext session, phext-shell will automatically write out a history of actions completed. This history is stored in phext itself, allowing you to track which commands were issued. This allows you to walk/share notes and learn from others in the way they discovered information!

Session Example
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