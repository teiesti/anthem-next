# Verification
The `verify` command uses the ATP `vampire` to automatically verify that some form of equivalence holds between two programs, or between a program and a target language specification.
These equivalence types are described below.


## Strong Equivalence
Strong equivalence is a property that holds for a pair of programs (`Π1`, `Π2`) if `Π1 U Π` has the same answer sets as `Π2 U Π`, for any program `Π`.
This property can be verified for mini-gringo programs by determining the equivalence of `τ*Π1` and `τ*Π2` within the HTA (here-and-there with arithmetic) deductive system.
This problem can be reduced to a first-order reasoning task by applying the gamma transformation, e.g. determining the equivalence of `γτ*Π1` and `γτ*Π2`.
The property can be automatically verified with the command
```
    anthem verify --equivalence strong p1.lp p2.lp
```


## External Equivalence
A mini-gringo program can contain "placeholders" which are symbolic constants that may be treated in a non-Herbrand way.
Such a program represents a class of programs

TODO


### Answer Set Equivalence
Answer set equivalence (which asserts two programs have the same answer sets) is a special case of external equivalence.
A user guide without placeholders, assumptions or input declarations, that contains every predicate in a pair of programs `(Π1, Π2)` as an output declaration, can be used to validate the answer set equivalence of `Π1` and `Π2`.

## Interpreting Anthem Output
Anthem will pass a series of problems to an ATP backend and report the status of each using the SZS status ontology.
If all problems are successfully verified, Anthem will report
```
    Success!
```
which indicates that the equivalence property holds.

Otherwise, Anthem will report
```
    Failure!
```
indicating that the equivalence property could not be verified.
Note that this is NOT a proof that the equivalence property does not hold.


## Problem Files vs End-to-end Use
Rather than invoking `vampire`, Anthem can produce a set of TPTP problem files that can be passed manually to a variety of ATPs.
If each problem is verified (the ATP reports a `Theorem` SZS status), then the verification can be considered successfully verified.
To invoke this option, add the `--no-proof-search` flag to a verification command, along with `--save-problems <DIR>` to save problem files to a directory of choice.
