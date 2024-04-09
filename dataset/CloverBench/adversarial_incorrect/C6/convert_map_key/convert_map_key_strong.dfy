method convert_map_key(inputs: map<nat, bool>, f: nat->nat) returns(r:map<nat, bool>)
  requires forall n1: nat, n2: nat :: n1 != n2 ==> f(n1) != f(n2)
  ensures forall k :: k in inputs <==> f(k) in r
  ensures forall k :: k in inputs ==> r[f(k)] == inputs[k]
{
  r:= map k | k in inputs :: f(k) := inputs[k];
}
