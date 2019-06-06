# filescan
A tool that scans the filesystem looking for possible duplicate files.

You invoke it like this:
```
./filescan ~
```

And it goes through everything in the ~ directory noting the size and a simple hash for every file. If the size and the hash are equal, it tells you that the two files are probably the same.  It also shows their size.  Errors come out on stderr. The possible duplicates appear on stdout.
