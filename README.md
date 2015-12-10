# avm [![Build Status](https://travis-ci.org/schultyy/avm.svg)](https://travis-ci.org/schultyy/avm)

## Motivation

Right now to install multiple versions of node.js there's [nvm](https://github.com/creationix/nvm) available. It does a decent job but there's one disadvantage: It's written in Shell. On the one hand that's nice because it's easy to install on Unix machines, but on the other hand it's not usable on Windows machines and Shell code is not easy to understand. At least for me.
Especially the latter reason is important for me. It is not that easy to find a person who can maintain Shell code and also it's not that easy to figure out where to look when something goes wrong.

Since Rust has become stable this year, I took the opportunity and began to write a replacement for nvm. It's called avm as abbreviation for "All version manager". In the future I want to be able to not only install node.js but also Ruby and Python on my machine in a convenient way. The other advantage is since Rust runs on many platforms, there's also the possibility to run avm on machines without Bash e.g. Windows.

## Installation

It installs from source, for that you need to have git and Rust stable installed.

To install avm on your system, run the following:

```bash
curl https://raw.githubusercontent.com/schultyy/avm/master/install.sh | bash
```

If Rust is not installed yet, visit [https://www.rust-lang.org/downloads.html](https://www.rust-lang.org/downloads.html) and download the version for your operating system.

After installation finished, you need to make sure that `avm` is in your `PATH`. For that you need to append the following line to either `~/.zshrc` or `~/.bash_profile`:

```bash
export PATH=~/.avm/:~/.avm/bin:$PATH
```

If you encounter the following compilation error on a Linux based system:

```bash
$ cargo build
#...
failed to run custom build command for `openssl-sys v0.6.6`
```

make sure that you have the following package installed:
Ubuntu:
```bash
$ sudo apt-get install libssl-dev
```
RHEL:
```bash
$ sudo yum install openssl-devel 
```

### Supported platforms

Right now, it is possible to run avm on the major Linux distributions and Mac OS X. There is no support for Windows right now. See [#33](https://github.com/schultyy/avm/issues/33) for details.

## Usage

Install a new node version:

```bash
$ avm install 4.1.2
```

Please note, that right now avm installs precompiled versions of Node.js. There is no supported yet for installing from source.

Use `4.1.2` by default:
```bash
$ avm use 4.1.2
```
Use your system node version:

```bash
$ avm use system
```

List all installed versions:

```bash
$ avm ls
```

Uninstall a version:

```bash
$ avm uninstall 4.1.2
```

