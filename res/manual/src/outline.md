# Proof Outline (.po)

A proof outline is understood as a sequence of steps for Anthem to take while attempting to construct a proof. When a step is verified successfully, the associated formula will be used as an axiom while attempting to prove subsequent steps. Typically, a proof outline is used to extend the set of axioms used within an (external equivalence) verification task.


Proof outlines consist of annotated formulas of three types:

1. definitions,
2. lemmas,
3. inductive lemmas.

For example,
```
    definition(universal)[completed_def_of_p]: forall X (p(X) <-> q(X) and not X = 0).
    lemma(forward)[int_q]: exists N$i q(N$i).
    inductive-lemma: forall N$i (N$i >= 0 -> q(N$i)).
```

These formulas can be annotated with any direction - it is the responsibility of the programmer to ensure that the claims they formulate make sense.
In the example above, the universal definition of `p/1` will be used as an axiom for deriving the program from the specification, and for deriving the specification from the program.
The inductive lemma will be proved first in both directions of the proof, and used as an axiom in subsequent steps.
The lemma will only be used in the forward direction: first, the lemma will be derived from the specification, then the lemma and the specification will be used as axioms for deriving the program.

### Lemmas
Lemmas have a general form

```
    lemma(direction)[name]: F.
```

where `F` is a target language formula. Within lemmas a formula `F` with free variables is interpreted as the universal closure of `F`.

### Inductive Lemmas
Inductive lemmas have a general form
```
    inductive-lemma(direction)[name]: forall X N ( N >= n -> F(X,N) ).
```

where n is an integer, `X` is a tuple of variables,`N` is an integer variable and `F` is a target language formula. As with lemmas, a formula with free variables is understood as an abbreviation for the universal closure of said formula.

Within a proof outline, an inductive lemma is interpreted as an instruction to prove two conjectures:

1. `forall X ( F(X,n) )`
2. `forall X N ( N >= n and F(X,N) -> F(X,N+1) )`

If both the first (the base case) and the second (the inductive step) conjectures are proven, then the original formula is treated as an axiom in the remaining proof steps.

### Definitions
Definitions are treated similarly to assumptions - they are assumed to define the extent of a new predicate introduced for convenience within a proof outline. They have the general form
```
    definition(direction)[name]: forall X ( p(X) <-> F(X) ).
```
where

1. `X` is a tuple of distinct integer or general variables
2. `p` is a totally fresh predicate symbol (it doesn’t occur outside of the proof outline)
3. `F` is a target language formula with free variables `X` that doesn’t contain atoms whose predicate symbol is `p`.

A sequence of definitions is valid if any definitions `p` used within each `F` were defined previously in the sequence. Intuitively, a definition should not depend on definitions that have not yet been defined. Thus, substituting the RHS for the LHS is always possible, and we could expand the body of the last definition by replacing any occurrences of previously defined definitions with their corresponding RHS.

Anthem will produce warnings about the following cases:

1. A definition where the defined predicate contains a variable that does not occur in the RHS is likely a mistake.
2. A lemma referencing a definition that hasn’t yet been defined in the proof outline is poor style.


### A Complete Example

The external equivalence of the Primes Programs can be verified with the assistance of the following proof outline:
```
    definition: forall I$ N$ (sqrt(I$,N$) <-> I$ >= 0 and I$ * I$ < N$ <= (I$+1) * (I$+1) ).
    lemma: sqrt(I$,N$) and (I$+1) * (I$+1) <= N$+1 -> sqrt(I$+1,N$+1).
    inductive-lemma: N$ >= 0 -> exists I$ sqrt(I$,N$).
    lemma: b$i >= 1 -> (sqrtb(I$) <-> sqrt(I$, b$i)).
    lemma: I$ >= 0 and J$ >= 0 and I$*J$ <= b and sqrtb(N$) -> I$ <= N$ or J$ <= N$.
```
