# User Guide (.ug)

A user guide contains input declarations, output declarations, and annotated formulas with the assumption role. An input declaration is a predicate or a placeholder. For example, the following are valid input declarations.

* input: p/0.
* input: edge/2.
* input: a.
* input: n -> integer.

Collectively, these lines denote that `p/0` and `edge/2` are input predicates, that `a` is an object-sorted placeholder, and that `n` is an integer-sorted placeholder. Anthem will throw an error if two placeholders with the same name are declared with different sorts.

### Placeholders

Syntactically, a placeholder is a symbolic constant. When an io-program `P` containing a symbolic constant `n` is paired with a user guide specifying `n` as a placeholder, every occurrence of `n` within `P` will be replaced by a zero-arity function constant of the specified sort. In the example above, `a` will be replaced by `a$g`, and `n` will be replaced by `n$i`. Placeholders are replaced in a similar fashion within specifications, proof outlines, and user guide assumptions. For example, within the context of a user guide containing the declaration

* input: n -> integer.

the (simplified) formula representation of the following rule

* p(X) :- X = 1..n.

would be

* forall (X) ( p(X) <-> exists I$i (1 <= I$i <= n$i and X = I$i) ).

### Input & Output Predicates

Input and output predicates are public predicates - all other predicates are considered private to the program. Input predicates are those predicates by which input data for the program are encoded. For example, the graph coloring program expects a set of `edge/2` and `vertex/1` facts encoding a graph and a set of colors (`color/1` facts) thus we pair that program with the user guide

* input: vertex/1.
* input: edge/2.
* input: color/1.
* output: color/2.

Output predicates function similarly to the `#show` directive in CLINGO. The extent of the output predicates define the external behavior of a program. In the graph coloring example, the external behavior is defined by the `color/2` predicate (mapping vertices to colors). Conversely, `aux/1` is the only private predicate.

### Assumptions

The only type of annotated formula accepted by user guides are assumptions. These assumptions are intended to define an acceptable class of inputs. Thus, they should not contain output symbols (this will trigger an error).

### Answer Set Equivalence

Answer set equivalence (which asserts two programs have the same answer sets) is a special case of external equivalence. A user guide without placeholders, assumptions or input declarations, that contains every predicate in a pair of programs `(P1, P2)` as an output declaration, can be used to validate the answer set equivalence of `P1` and `P2`.
