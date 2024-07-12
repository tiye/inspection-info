## Inspect Info

> a bunch of commands for dev informations.

### Usages

Install it with cargo install git source.

```bash
in help
```

##### IP address

```bash
in ip
10.143.35.141
```

```bash
in ip -d
lo0:	127.0.0.1
lo0:	::1
lo0:	fe80::1
en0:	192.168.35.141
```

##### Copy file to clipboard

```bash
in cpfile Cargo.toml
Copiled 192 characters to clipboard
```

### List largest files

```bash
in large --min 1k --sort
5.6 MB ./target/release/deps/libserde-3f58fa041f951e55.rmeta
5.6 MB ./target/release/deps/libserde-8238757ab41c1ecb.rmeta
5.7 MB ./target/release/deps/libserde-3f58fa041f951e55.rlib
5.7 MB ./target/release/deps/libserde-8238757ab41c1ecb.rlib
```

### Dir mark

```bash
in dir add demo
in dir jump demo
in dir ls
```

Helper function `ig` for jump:

```bash
function gg {
  eval "in dir jump $1"
  local target=/tmp/inspection-bashmarks-jump-target
  if test -f $target; then
    cd "$(cat $target)"
    ls -pG
  fi
}
```

### License

MIT
