## Symlink Your Damn Files

### Description:

_sydf_ is a small bash utility for managing files. Main purpose of this utility
is to make sharing and syncing dotfiles across devices as easy as possible. It is intended to be used in conjunction with git, syncthing, borgbackup,
nextcloud and other programs.

### Installation

Run these commands in your terminal to set up _sydf_ system-wide:
```sh
# Install latest version
curl -L "https://raw.githubusercontent.com/filiparag/sydf/master/sydf" > /tmp/sydf

# Install stable release 0.1
curl -L "https://github.com/filiparag/sydf/releases/download/0.1/sydf" > /tmp/sydf

chmod +x /tmp/sydf
sudo mv /tmp/sydf /usr/bin/sydf

```

### Usage
