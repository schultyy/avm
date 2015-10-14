# avm
all version manager -- Right now for node.js only

This installs your node.js versions of choice on your machine. avm uses `~/.avm` for everything, so you can operate without `sudo`.

## Usage

Install a new node version:

```bash
$ avm install 4.1.2
```

Use `4.1.2` by default:
```bash
$ avm use 4.1.2
```

List all installed versions:

```bash
$ avm ls
```
