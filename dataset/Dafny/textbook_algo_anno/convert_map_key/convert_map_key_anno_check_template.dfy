ghost predicate pre_original(inputs: map<nat, bool>, f: nat->nat,r:map<nat, bool>){
  forall n1: nat :: forall n2:nat :: (n1 != n2) ==> (f(n1) != f(n2))
}

ghost predicate pre_gen(inputs:map<nat, bool>, f: nat->nat,r:map<nat, bool>)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(inputs: map<nat, bool>, f: nat->nat, r: map<nat, bool>, n1: nat,n2: nat)
  ensures pre_original(inputs,f,r) <==> pre_gen(inputs,f,r)
{
}

ghost predicate post_original(inputs: map<nat, bool>, f: nat->nat,r:map<nat, bool>,n1: nat,n2: nat)
  requires pre_original(inputs,f,r){
  ( forall k :: k in inputs <==> f(k) in r) &&
  ( forall k :: k in inputs ==> r[f(k)] == inputs[k])
}

ghost predicate post_gen(inputs: map<nat, bool>, f: nat->nat,r:map<nat, bool>,n1: nat,n2: nat)
  requires pre_original(inputs,f,r){
  true // (#POST) && ... (#POST)
}

lemma post_eq(inputs: map<nat, bool>, f: nat->nat,r:map<nat, bool>,n1: nat,n2: nat)
  requires pre_original(inputs,f,r)
  requires pre_gen(inputs,f,r)
  ensures post_original(inputs,f,r,n1,n2 ) <==> post_gen(inputs,f,r,n1,n2 )
{
}

