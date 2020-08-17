# readelf.rs

`readelf`コマンドをRustで実装した。

## Features

- [x] `-h`: ELFヘッダー
- [x] `-l`: プログラムヘッダー
- [x] `-S`: セクションヘッダー
- [x] `-s`: シンボル

## How To Use

```bash
$ readelf <command> <file>
```

command: `all` `header` `pheader` `sheader` `symbol`
