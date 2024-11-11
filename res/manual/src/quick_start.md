Welcome! 
To get started with program verification, install `anthem` and cd into the `anthem` directory.
The res/examples folder contains two types of verification tasks: external equivalence and strong equivalence.
Within the respective directories of these tasks are a number of simple or well-studied problems.
We recommend visiting the README for every problem first -- it usually provides usage instructions and literature pointers for theoretical background.

For example, the following command 
```
    anthem verify --equivalence strong res/examples/strong_equivalence/choice/choice.{1.lp,2.lp} -m 2 --no-simplify -t 30
```
verifies the strong equivalence of choice.1.lp and choice.2.lp using a single instance of Vampire parallelized with 2 cores.
Additionally, this command disables `anthem`'s formula simplification algorithm, and subjects every task passed to `vampire` to a 30 second timeout.

You should see the following output:
```
    > Proving forward_0...
    Axioms:
        forall X1 (hq(X1) -> tq(X1))
        forall X1 (hp(X1) -> tp(X1))
        forall V1 X ((V1 = X and exists Z (Z = X and hp(Z)) and not not tq(V1) -> hq(V1)) and (V1 = X and exists Z (Z = X and tp(Z)) and not not tq(V1) -> tq(V1)))

    Conjectures:
        forall V1 X ((V1 = X and (exists Z (Z = X and hp(Z)) and exists Z (Z = X and not not tq(Z))) -> hq(V1)) and (V1 = X and (exists Z (Z = X and tp(Z)) and exists Z (Z = X and not not tq(Z))) -> tq(V1)))

    > Proving forward_0 ended with a SZS status
    Status: Theorem

    > Proving backward_0...
    Axioms:
        forall X1 (hq(X1) -> tq(X1))
        forall X1 (hp(X1) -> tp(X1))
        forall V1 X ((V1 = X and (exists Z (Z = X and hp(Z)) and exists Z (Z = X and not not tq(Z))) -> hq(V1)) and (V1 = X and (exists Z (Z = X and tp(Z)) and exists Z (Z = X and not not tq(Z))) -> tq(V1)))

    Conjectures:
        forall V1 X ((V1 = X and exists Z (Z = X and hp(Z)) and not not tq(V1) -> hq(V1)) and (V1 = X and exists Z (Z = X and tp(Z)) and not not tq(V1) -> tq(V1)))

    > Proving backward_0 ended with a SZS status
    Status: Theorem

    > Success! Anthem found a proof of equivalence.
```

This task asks `vampire` to verify two conjectures, named forward_0 and backward_0.
For both conjectures, `vampire` reports that it was able to derive the conjecture from the axioms (SZS Status: Theorem) within the time limit.
Since both directions of the proof (forward and backward) were verified, `anthem` reports that a proof of equivalence has been found.
Thus, the first program can be derived from the second, and vice versa, satisfying the conditions for strong equivalence.

Try another example:
```
    anthem verify --equivalence external res/examples/external_equivalence/primes/simple/primes.{1.lp,2.lp,ug} -t 30 -m 4 --no-eq-break
```
This one is harder. 
`anthem` fails to verify that the two programs are externally equivalent with respect to the assumptions of primes.ug within the time limit.
But, this does not necessarily mean the proof does not exist.

If we re-enable the equivalence breaking feature of `anthem`, (on by default) which divides conjecures of the form
```
    forall X ( F(X) <-> G(X) )
```
into a pair of conjectures
```
    forall X ( F(X) -> G(X) )
    forall X ( G(X) -> F(X) )
```
`anthem` successfully verifies the external equivalence of these programs.

If you have a really hard problem (for instance, external_equivalence/primes/complex or external_equivalence/division) you will likely need a proof outline (see the reference manual).
