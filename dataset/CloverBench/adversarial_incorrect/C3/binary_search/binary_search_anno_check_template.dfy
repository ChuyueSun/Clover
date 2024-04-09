predicate pre_original(a: array<int>,key: int,n: int)
  reads a
{
  ( forall i,j :: 0<=i<j<a.Length ==> a[i]<=a[j])
}

predicate pre_gen(a: array<int>,key: int,n: int)
  reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>,key: int,n: int)
  ensures pre_original(a,key,n ) <==> pre_gen(a,key,n )
{
}

predicate post_original(a: array<int>,key: int,n: int)
  requires pre_original(a,key,n)
  reads a
{
  ( 0<= n <=a.Length) && ( forall i :: 0<= i < n ==> a[i]<key) &&
  (n == a.Length ==> forall i :: 0 <= i < a.Length ==> a[i] < key)
}

predicate post_gen(a: array<int>,key: int,n: int)
  requires pre_original(a,key,n)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>,key: int,n: int)
  requires pre_original(a,key,n )
  requires pre_gen(a,key,n )
  ensures post_original(a,key,n ) <==> post_gen(a,key,n )
{
}

