predicate pre_original(a: array2<int>,key: int,m:int,n:int)
  reads a
{
  ( forall i,j,j'::0<=i<a.Length0 &&  0<=j<j'<a.Length1 ==> a[i,j]<=a[i,j']) &&
  ( forall i,i',j::0<=i<i'<a.Length0 &&  0<=j<a.Length1 ==> a[i,j]<=a[i',j]) &&
  ( exists i,j :: 0<=i<a.Length0 &&  0<=j<a.Length1 &&  a[i,j]==key)
}

predicate pre_gen(a: array2<int>,key: int,m:int,n:int)
  reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array2<int>,key: int,m:int,n:int)
  ensures pre_original(a,key,m,n ) <==> pre_gen(a,key,m,n )
{
}

predicate post_original(a: array2<int>,key: int,m:int,n:int)
  requires pre_original(a,key,m,n)
  reads a
{
  ( 0<=m<a.Length0) && ( 0<=n<a.Length1) && ( a[m,n]==key)
}

predicate post_gen(a: array2<int>,key: int,m:int,n:int)
  requires pre_original(a,key,m,n)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array2<int>,key: int,m:int,n:int)
  requires pre_original(a,key,m,n )
  requires pre_gen(a,key,m,n )
  ensures post_original(a,key,m,n ) <==> post_gen(a,key,m,n )
{
}

