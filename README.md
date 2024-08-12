phext shell
-----------
* a shell that is phext-aware
* keeps track of your current scroll by coordinate
* allows programs to pass hierarchical information between processes
* 10 operating modes (remaps <ENTER> to apply phext breaks)
  * allows you to chain outputs in a hierarchical manner
  * F2 -> Line Break
  * F3 -> Scroll Break
  * F4 -> Section Break
  * F5 -> Chapter Break
  * F6 -> Book Break
  * F7 -> Volume Break
  * F8 -> Collection Break
  * F9 -> Series Break
  * F10 -> Shelf Break
  * F11 -> Library Break
  * F12 -> Toggle auto-scrolling

1.1.1/1.1.1/1.1.1> hello-phext<LB>

Result: All output from the `hello-phext` process will be collected on the scroll starting at 2.1.1/1.1.1/1.1.1.
No additional programs can be started from this node, but we can change our current scroll with the `cs` command.

2.1.1/1.1.1/1.1.1> ls
ERROR: `hello-phext` is currently running. Switch to another scroll context to run a new program.

2.1.1/1.1.1/1.1.1> cs 1.1.1/1.1.1/1.1.2

Result: The user's I/O mount point will be adjusted to the given coordinate, which is not generating any output currently.

1.1.1/1.1.1/1.1.2> cs 1.1.1/1.1.1/1.1.1

Result: The user's I/O mount point will return to the root node, which is also not producing any output currently.