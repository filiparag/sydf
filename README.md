## Symlink Your Damn Files

### Description

_sydf_ is a small bash utility for managing files. Main purpose of this utility
is to make sharing and syncing dotfiles across devices as easy as possible. It is intended to be used in conjunction with git, syncthing, borgbackup,
nextcloud and other programs.

### Installation

**Arch User repository**

You can install the latest version of _sydf_ from AUR using your favourite package manager:
```sh
yay -S sydf
```

**Manual**

Run these commands in your terminal to set up _sydf_ system-wide:
```sh
# Install latest version
curl -L "https://raw.githubusercontent.com/filiparag/sydf/master/sydf" > /tmp/sydf

# Install stable release 0.2
curl -L "https://github.com/filiparag/sydf/releases/download/0.2/sydf" > /tmp/sydf

chmod +x /tmp/sydf
sudo mv /tmp/sydf /usr/bin/sydf

```

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

Show a list of all files and directories managed by _sydf_.
```sh
sydf list
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

**Ignore list**

If you want to have some files inside _sydf_'s working directory ignored by _sydf_, you can add them into the `.sydf` file. _sydf_ will not be able to remove lines containing ignored files, but be wary that the same file is used for directory tracking and it might change over time. Every ignored file rule has to follow this syntax:
```
!<DIRECTORY>#
!<FILE>#
```
All ignored paths are relative to the _sydf_'s working directory. Do not leave the trailing slash at the end of directory paths, as it won't work!
