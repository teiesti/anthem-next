# Output File Format

Variants of the `--verify` command can produce problem files that can be passed to an ATP.
For example, the command
```
anthem verify --equivalence external res/examples/external_equivalence/primes/simple/primes.{1.lp,2.lp,ug} --no-proof-search --save-problems ./
```

produces a set of TPTP problems files in the current directory (`./`) without invoking an ATP to verify them.
In this case, verifying each problem file amounts to proving the external equivalence of the programs `primes.1.lp` and `primes.2.lp` under the assumptions of `primes.ug`.
