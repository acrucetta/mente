## Linux

*Everything is a file in Unix*

### File Systems
File System Commands:
- `ls`
	- -a all files
	- -d directory
	- -c classify
	- -h human readable
	- -r reverse
	- -S sort by file size
- `file`
- `less`

Directories
- bin - binaries must be present for the system to boot and run
- boot - contains the kernel
- dev - device nodes
- etc - system wide config files
- home - normal config
- lib - shared libraries
- media - mount points for removable media
- mnt - removable storage devices
- opt - optional software
- root - home
- tmp - temp files
- user - directory tree by user

Symbolic Links: 
- soft link: a file can be reference by multiple names
	- e.g., a program needs access to a shared resource
- hard links: allow files to have multiple names

### Manipulating Files and Directories

Commands:
- cp - copy files
	- `cp file 1 file 2` - simple
	- `cp -I file1 file2` - interactive mode
	- `cp file1 file2 dir1` - copy file 1 and 2 to dir1
	- `cp dir1/* dir2` - all files in dir1 copied to dir2
- mv - move and rename files
	- `mv file1 file2` - moves file 1 to file 2; if file 2 exists it is overwritten. If not, it's created
	- `mv file1 file2 dir1` - moves file 1 and 2 into dir 1
- mkdir - create dirs
- rm - remove and dirs files
	- `rm -r dir` - deletes all contents in directory
	- `rm *.html` - removes all files with the extension *.html
- ln - create hard and symbolic links
	- `ln file link` - link file to link
	- `ln -s file link` - hard link file to link
		- *A hard link is indistinguishable from the file itself. Unlike a directory list containing a symbolic link, a directory list containing a hard link shows no special indication of the link.*

### Working with Commands

Commands:
- type - how a command name is interpreted
- which - show the executable program
- man - display man page
- apropos - list of appropriate commands
- info - command info entry
- whatis - brief description of command
- alias - alias for command
	- `alias foo='cd /usr ; ls ; cd -'`


### Redirection commands

Commands:
- cat - concatenate files
- sort - sort text
- uniq - report or omit repeated lines
- wc - print newline word and bytecounts
- grep - print lines matching a pattern
- head - first part of file
- tail - lass part of file
- tee - read from stdin and write to stdout

Piping command example
- `ls /bin /usr/bin | sort | uniq | grep zip`

### Echo Command

- Arithmetic Expansion: `echo $((2+2))`
- Brace Expansion: `echo Front-{A,B,C}-Back`
- Range of Numbers/Letters
	- `echo Number {1...5}`
	- `echo {Z..A}`

### Keyboard Tricks

- clear - clear the screen
- history - history list of commands

