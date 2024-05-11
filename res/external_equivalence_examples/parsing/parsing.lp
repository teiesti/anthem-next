edge(X,Y) :- X < 5, Y < n.
aux(X) :- edge(X,Y), not t.
reachable(Z,V) :- edge(Z,Var), edge(Var, V), aux(Var).
