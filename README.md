# filescan
A tool that scans the filesystem looking for possible duplicate files.

You invoke it like this:
```
./filescan ~
```

And it goes through everything in the ~ directory noting the size and a simple hash for every file. If the size and the hash are equal, it tells you that the two files are probably the same.  It also shows their size.  Errors come out on stderr. The possible duplicates appear on stdout.

```
./filescan ~ >dups.txt 2>errs.txt
```

If you hit control-c while it is running, it will finish the directory it is scanning, quit scanning, and output the results.

```
2.82 MB:
    /Users/aaron/Properties/1989CollegeAve/Title Exam.pdf
    /Users/aaron/Desktop/1989 College/Title Exam.pdf
2.17 MB:
    /Users/aaron/Downloads/IMG_20190105_082502.jpg
    /Users/aaron/Movies/iMovie Library.imovielibrary/First Day/Original Media/IMG_20190105_082502.jpg
```

