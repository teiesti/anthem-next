# Absolute Lemmas

The following formulas have been established as consequences of the standard preamble.
They are called "absolute lemmas" due to their origin as helpful lemmas that can apply to any problem.
For difficult problems, it may help to add them as universal axioms instead of lemmas or inductive lemmas.
Some must be used jointly with definitions (e.g. `sqrt/1`).

```
    axiom: I$ >= 0 -> (I$+2)*(I$+2) > (I$+1)*(I$+1) + 1.
```

```
    definition: forall I$ N$ (sqrt(I$,N$) <-> I$ >= 0 and I$*I$ <= N$ < (I$+1)*(I$+1)).
    lemma: sqrt(I$,N$) and (I$+1)*(I$+1) <= N$+1 -> sqrt(I$+1,N$+1).
    inductive-lemma: N$ >= 0 -> exists I$ sqrt(I$,N$).
```

```
    lemma: forall X, I$, J$ ( (I$ > 1 and J$ > 1 and X = I$ * J$) -> (I$ <= X and J$ <= X) ).
```
