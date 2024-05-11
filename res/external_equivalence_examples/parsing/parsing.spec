assumption: n$i > 5 -> t.
assumption(backward): forall X (edge(X, n$i) <-> t).
spec[spec_1]: reachable(5,6).
spec(forward)[spec_2]: forall X Y ( reachable(X,Y) <-> edge(X,Y) ).
