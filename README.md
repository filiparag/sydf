## Symlink Your Damn Files

### Description

_sydf_ is a bash utility for managing files. Main purpose of this utility is to make sharing and syncing dotfiles across devices as easy as possible by keeping them in a single directory. It is intended to be used in conjunction with git, syncthing, borgbackup, nextcloud and other programs.

### Installation

**Arch User repository**

You can install the latest version of _sydf_ from AUR using your favourite package manager:
```sh
yay -S sydf
```

**Manual**

Run these commands in your terminal to set up _sydf_ system-wide:
```bash
# Download latest stable release
REL=$(curl -LsI 'https://github.com/filiparag/sydf/releases/latest' | grep -Po 'tag\/\K(\S+)')
curl -L "https://github.com/filiparag/sydf/releases/download/$REL/sydf" > /tmp/sydf

# Download latest development version - not recommended
curl -L "https://raw.githubusercontent.com/filiparag/sydf/master/sydf" > /tmp/sydf

chmod +x /tmp/sydf
sudo mv /tmp/sydf /usr/bin/sydf
```
Warning: installing the latest development version as opposed to a stable release might result in broken features and data loss! Proceed with caution.

### Usage

**Initialization**

Similar to a git repository, _sydf_ needs a working directory for file managment. Default directory is `$HOME/.sydf`.
```sh
sydf init <DIRECTORY>
```
**Adding files and directories**

Adding files to _sydf_ means that they will get moved to its working directory and a symbolic link will be placed in their original place. Linking a directory will move it same as it was a regular file, so its content won't be symlinked individually. Plan ahead and decide if moving individual files inside directory is better suited for your needs - wildcards are supported in both add and remove commands.
```sh
sydf add <FILE1> <DIRECTORY> <FILE2>...
```

**Removing files and directories**

Removing a file or a directory from _sydf_ means reversing what adding them did. Symbolic link is destroyed and the file is put back into its original place. This operation will not harm any of your files, unlike similar command for git.
```sh
sydf remove <FILE> <DIRECTORY>...
```

**List**

Show a list of files and directories managed by _sydf_.
```sh
# List managed directories
sydf list dirs

# List files outside managed directories
sydf list files

# List all managed files and directories
sydf list all

# List files and directories inside .old directory
sydf list old
```

**Hook**

Hooking will attempt to link all files and directories inside _sydf_'s working directory to their respective places on system. This comand is intended to be used for transfer of your managed files to another machine or sharing with another user. If a file already exists in targeted place, it will be safely moved to `.old` directory inside _sydf_'s working directory.
```sh
sydf hook
```

**Unhook**

Unhooking will replace all symbolic links created by _sydf_ with corresponding files and directories. Contents of _sydf_'s working directory will stay unaffected by this operation.
```sh
sydf unhook
```

**Revert**

Revert can be used to completely undo all actions performed on the filesystem outside _sydf_'s working directory. All created directories will be removed unless they contain data not managed by _sydf_. Reverting will not work if files are unhooked beforehand.
```sh
sydf revert
```
Warning: any modification of the `.old` directory inside of the working folder might break reversion process!

**Ignore**

If you want to have some unmanaged paths like `.git` inside the working directory, you can add them to the ignore list. All ignored paths include their subpaths automatically.
```sh
# List ignored paths
sydf ignore

# Add paths to ignore list
sydf ignore add <PATH> ...

# Remove paths from ignore list
sydf ignore remove <PATH> ...
```
All ignored paths are relative to the _sydf_'s working directory.
