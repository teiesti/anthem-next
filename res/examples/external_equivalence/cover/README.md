# `cover`

## Usage
To run the original program-to-specification verification task, use

```
mkdir -p out/p2s
anthem verify --equivalence=external --out-dir out/p2s cover.spec cover.lp cover.ug
```

To run the program-to-program verification task against a new program with a symmetry breaking constraint, use

```
mkdir -p out/p2p
anthem verify --equivalence=external --out-dir out/p2p cover.lp cover_sym_break.lp cover.ug
```

## Origin
The original program-to-specification verification task was taken from

> Jorge Fandinno, Vladimir Lifschitz, Patrick Lühne, Torsten Schaub:
> Verifying Tight Logic Programs with anthem and vampire. TPLP 20(5): 735-750 (2020).
> https://doi.org/10.1017/S1471068420000344

The program-to-program verification task was taken from

> Yuliya Lierler:
> Verification of Refactoring in Answer Set Programming. FLOPS 2024.
> https://conf.researchr.org/details/flops-2024/flops-2024-papers/3/Verification-of-Refactoring-in-Answer-Set-Programming
