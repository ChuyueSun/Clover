predicate pre_original(a:array<int>,j: nat,k: nat)
  reads a
{
  ( 0<=j < a.Length)
}

predicate pre_gen(a:array<int>,j: nat,k: nat)
  reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a:array<int>,j: nat,k: nat)
  ensures pre_original(a,j,k ) <==> pre_gen(a,j,k )
{
}

predicate post_original(a:array<int>,j: nat,k: nat)
  requires pre_original(a,j,k)
  reads a
{
  ( a[j] >= 60)
}

predicate post_gen(a:array<int>,j: nat,k: nat)
  requires pre_original(a,j,k)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a:array<int>,j: nat,k: nat)
  requires pre_original(a,j,k )
  requires pre_gen(a,j,k )
  ensures post_original(a,j,k ) <==> post_gen(a,j,k )
{
}

