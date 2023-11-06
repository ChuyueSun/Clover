predicate pre_original(a: array<int>){
  true
}

predicate pre_gen(a: array<int>){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>)
  ensures pre_original(a) <==> pre_gen(a )
{
}

twostate predicate post_original(a: array<int>)
  reads a
  requires pre_original(a){
forall i :: 0 <= i < a.Length ==> a[i] == old(a[a.Length - 1 - i])
}

twostate predicate post_gen(a: array<int>)
  reads a
  requires pre_original(a){
  true // (#POST) && ... (#POST)
}

twostate lemma post_eq(a: array<int>)
  requires pre_original(a )
  requires pre_gen(a )
  ensures post_original(a ) <==> post_gen(a )
{
}

