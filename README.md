[![Build Status](https://travis-ci.org/schultyy/avm.svg)](https://travis-ci.org/schultyy/avm)

# avm

This installs your node.js versions of choice on your machine. avm uses `~/.avm` for everything, so you can operate without `sudo`.

## Installation

To install avm on your system, run the following:

```bash
curl https://raw.githubusercontent.com/schultyy/avm/master/install.sh | bash
```

It installs from source, for that you need to have git and Rust stable installed.
If Rust is not installed yet, visit [https://www.rust-lang.org/downloads.html](https://www.rust-lang.org/downloads.html) and download
the version for your operating system.

After installation finished, you need to make sure that `avm` is in your `PATH`. For that you need to append the following line to either `~/.zshrc` or `~/.bash_profile`:

```bash
export PATH=~/.avm/:~/.avm/bin:\$PATH
```

If you encounter the following compilation error on a Linux based system:

```bash
$ cargo build
#...
failed to run custom build command for `openssl-sys v0.6.6`
```

make sure that you have the following package installed:

```bash
sudo apt-get install libssl-dev
```

### Supported platforms

Right now, it is possible to run avm on the major Linux distributions and Mac OS X. There is no support for Windows right now. See [#33](https://github.com/schultyy/avm/issues/33) for details.

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

Uninstall a version:

```bash
$ avm uninstall 4.1.2
```
