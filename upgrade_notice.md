# Upgrade notice

If you're coming from avm < 1.0, then you need to do some adjustments to your `~/.avm` directory.

1. create a `node` directory in `~/.avm`

```bash
$ mkdir -p ~/.avm/node
```

2. Move all existing directories which are in `~/.avm` into `node/`:

```bash
$ ls ~/.avm
bin 5.3.0 4.3.0 node
$ mv {bin,5.3.0,4.3.0} node/
```

3. Adapt the paths in your `PATH` variable

Before:
```bash
$ echo $PATH
~/.avm/bin:$PATH
```
After:
```bash
$ export PATH=~/.avm/node/bin:~/.avm/ruby/bin:$PATH
```
(You might want to put this in `.bashrc`, `.bash_profile` or `.zshrc` depending on your used Shell).

