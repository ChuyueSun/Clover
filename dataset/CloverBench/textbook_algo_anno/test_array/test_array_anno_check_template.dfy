predicate pre_original(a:array<int>,j: nat)
  reads a
{
  ( 0<=j < a.Length)
}

predicate pre_gen(a:array<int>,j: nat)
  reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a:array<int>,j: nat)
  ensures pre_original(a,j) <==> pre_gen(a,j )
{
}

twostate predicate post_original(a:array<int>,j: nat)
  requires pre_original(a,j)
  reads a
{
  ( a[j] == 60) && (forall k :: 0 <= k < a.Length && k != j ==> a[k] == old(a[k]))
}

twostate predicate post_gen(a:array<int>,j: nat)
  requires pre_original(a,j)
  reads a
{
  true // (#POST) && ... (#POST)
}

twostate lemma post_eq(a:array<int>,j: nat)
  requires pre_original(a,j)
  requires pre_gen(a,j )
  ensures post_original(a,j) <==> post_gen(a,j )
{
}

