# q

a tiny cli that sorts json keys alphabetically. about 300 lines. i wrote it because jq's `-S` flag kept losing my comments.

## install

`cargo install q`

## use

```
q in.json > out.json
```

## notes

works on files up to ~200mb on my m2. bigger files probably work, i haven't checked. ymmv.
