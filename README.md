# anthem

These are usage instructions for the version deployed on anya (May 16th, 2024).

## Translate

Translating a logic program into a first-order theory:
```sh
$ ./anthem translate <path>.lp --with tau-star
$ ./anthem translate <path>.lp --with completion
$ ./anthem translate <path>.lp --with gamma 
```

Translating a first-order theory into another first-order theory:
```sh
$ ./anthem translate <path>.spec --with completion
$ ./anthem translate <path>.spec --with simplify
```

By default, simplifications are NOT applied. To enable simplifications, add a `--simplify` or `-s` flag. For example,
```sh
$ ./anthem translate <path>.lp --with tau-star -s
```

## Verify

Verifying a logic program against a specification:
```sh
$ ./anthem verify <specification path>.spec <program path>.lp <user guide>.ug --direction forward
$ ./anthem verify <specification path>.spec <program path>.lp <user guide>.ug --direction backward
```
The preceding pair of calls is equivalent to 
```sh
$ ./anthem verify <specification path>.spec <program path>.lp <user guide>.ug --direction universal
```
or, since universal is the default proof direction, to
```sh
$ ./anthem verify <specification path>.spec <program path>.lp <user guide>.ug
```

Verifying the external equivalence of two logic programs:
```sh
$ ./anthem verify <original>.lp <alternative>.lp <user guide>.ug
```

For convenience, you can use the `verify-alt` command in conjunction with a problem directory. For example, the primes problem is arranged as follows:
```sh
primes/
  primes.1.lp
  primes.2.lp
  primes.ug
  primes.help.spec
```
The alphabetically first program (in this case, `primes.1.lp`) is treated as the specification/original program. The commands
```sh
$ ./anthem verify primes/primes.1.lp primes/primes.2.lp primes/primes.ug primes/primes.help.spec
$ ./anthem verify-alt primes
```
can be used interchangeably.

By default, simplifications and equivalence breaking are applied. To disable these features, add the flags `--no-simplify` and `--no-break`. For example,
```sh
$ ./anthem verify-alt primes --no-simplify --no-break
```

The general form of a verify command is:
```sh
$ ./anthem verify <specification path> <program path>.lp <user guide>.ug <proof outline>.help.spec --decomposition <strategy> --time-limit <seconds> --out-dir <directory> --no-simplify --no-break
```
where `strategy` is "independent" or "sequential", `seconds` specifies how long `vampire` can work on each conjecture, `directory` specifies where/if the generated sequence of TPTP problem files should be stored, and the `no-simplify` and `no-break` flags disable simplifications and equivalence breaking.

## Writing Proof Outlines
`anthem` supports lists of annotated formulas with a ".help.spec" extension. Annotated formulas have the syntax `role(direction): formula`. Valid proof outline roles are lemmas, definitions, and inductive lemmas.
For example:
```sh
lemma(forward): exists N$i (p(N$i) and N$i > 0).
lemma(backward): exists N$i (q(N$i) and N$i < 0).
lemma(universal): forall X M$i N$i ( M$i > 1 and N$i > 1 and X = M$i * N$i -> M$i <= X and N$i <= X ).
```
`anthem` will attempt to prove the first lemma as the first step of the forward direction of the proof (deriving the program from the specification). Similarly, it will attempt to prove the second lemma as the first step of the backward direction of the proof (deriving the specification from the program). The last lemma will be the first step attempted in both directions. It can also be written as
```sh
lemma: forall X M$i N$i ( M$i > 1 and N$i > 1 and X = M$i * N$i -> M$i <= X and N$i <= X ).
```
Definitions and inductive lemmas are structured similarly:
```sh
definition: forall X (p(X) <-> exists Y q(X,Y)).
inductive-lemma: forall N$i (N$i >= 0 -> p(N$i)).
```

Providing `anthem` with a proof outline instructs the system to attempt to prove the sequence of basic/inductive lemmas in the proof outline sequentially. That is, each newly proven lemma will be added as an axiom to the set of premises used to derive the next lemma in the sequence. After every step in the proof outline is verified in this way, the lemmas and definitions will be treated as additional premises (axioms) during the remainder of the verification process (regardless of decomposition strategy).


## Extra Information
Prefacing a command with `RUST_LOG=INFO` will provide additional information like system runtimes. For example, running
```sh
$ RUST_LOG=INFO ./anthem verify-alt examples/primes
```
will display summaries (premises and conjectures) of each problem in the problem sequence and the time in milliseconds required to prove each conjecture.
