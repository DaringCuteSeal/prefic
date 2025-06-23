# Prefic

This is Prefic, a tool to decipher/cipher to/from variable-width [prefix code](https://en.wikipedia.org/wiki/Prefix_code). The dictionary is hardcoded for now, but you can simply change it if you want. Custom provided dictionary after compile-time coming soon (maybe).

Examples:
```
915678723217872321356816789191214321121361632158278721816299932172136321413614913212578035826913210149653915678143432103609757814341532178321356816136912321915678723219156232140965332101443219156232140965332182121361623213696533211365878363587804232172078147232170081835809121499932101570150321678919653321582712583216136363581891216321726782146232113658965336358780432182601872232141367814343210149653915678143432125872232135687858583213607023213623211222583218049993218189132172911849653781434321356813618584321560712321360423217891321356813616722999321721361515
```

```
358582072232135687872563213623215818670999321058915613618345632178363217218162321783632170781440321613613670243215836013615321818913217832172917858583215607123211425780913219653201615
```


# Installation

I don't know, just clone it and use `cargo` to build it.

```sh
git clone https://github.com/DaringCuteSeal/prefic.git
cd prefic
cargo build --release
```

and get the binary from the `target` directory.

# Usage
```sh
Usage: prefic <COMMAND>

Commands:
  cipher    Cipher an input string
  decipher  Decipher an input string
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Prefic acts upon input stream, not arguments given. This is useful and definitely follows the unix philosophy ;-)

Quick demo:
```sh
$> echo "hello this is some text to encode" | prefic cipher | prefic decipher
hello this is some text to encode
```


# Todo

- [x] Implement prefix tree-based query
- [ ] Cache prefix tree to filesystem for faster access
- [ ] Allow for custom dictionary
