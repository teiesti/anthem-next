# Translation

Anthem is primarily a translational tool - it transforms ASP programs into theories written in the syntax of first-order logic.
Additional transformations within this syntax can sometimes produce theories whose classical models coincide with the stable models of the original program.

## The mini-gringo Dialect
The mini-gringo dialect is a subset of the language supported by the answer set solver clingo.
It has been extensively studied within the ASP literature as a theoretical language whose semantics can be formally defined via transformations into first-order theories interpreted under the semantics of here-and-there (with arithmetic).

A mini-gringo program consists of three types of rules: choice, basic, and constraint:

```
    {H} :- B1, ..., Bn.
    H   :- B1, ..., Bn.
        :- B1, ..., Bn.
```

where H is an atom and each Bi is a singly, doubly, or non-negated atom or comparison.


## The Target Language
The formula representation language (often called the "target" of ASP-to-FOL transformations) is a many-sorted first-order language.
Specifically, all terms occurring in a mini-gringo program belong to the language's supersort, `general` (abbreviated `g`).
It contains two special terms, `#inf` and `#sup`, representing the minimum and maximum elements in the total order on general terms.
Numerals (which have a one-to-one correspondence with integers) are a subsort of this sort, they belong to a sort referred to as `integer` (abbreviated `i`).
All other general terms are symbolic constants, they belong to the `symbol` sort (abbreviated `s`).

Variables ranging over these sorts are written as `name$sort`, where `name` is a capital word and `sort` is one of the sorts defined above.
Certain abbreviations are permitted; the following are examples of valid variables:

```
    V$general, V$g, V
    X$integer, X$i, X$
    Var$symbol, Var$s
```

These lines represent equivalent ways of writing a general variable named `V`, an integer variable named `X`, and a symbol variable named `Var`.



## The Formula Representation Transformation (tau*)

The transformation referred to in the literature as `tau*` (`τ*`) is a collection of transformations from mini-gringo programs into the syntax of first-order logic, taking special consideration for the fact that while an ASP term can have 0, 1, or many values, a first-order term can have only one.
In the presence of arithmetic terms or intervals, such as `1/0` or `0..9`, this introduces translational complexities.
Interested readers should refer to ??? for details.

The tau* transformation is fundamental to Anthem.
For a mini-gringo program `Π`, the HTA (here-and-there with arithmetic) models of the formula representation `τ*Π` correspond to the stable models of `Π`.
Furthermore, additional transformations can, in certain cases, produce formula representations within the same language whose classical models capture the behavior of `Π`.


## Transformations Within the Target Language

#### Gamma


#### Completion
