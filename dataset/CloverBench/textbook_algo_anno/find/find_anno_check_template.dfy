predicate pre_original(a: array<int>,key: int,index: int)
  reads a
{
  true
}

predicate pre_gen(a: array<int>,key: int,index: int)
  reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: array<int>,key: int,index: int)
  ensures pre_original(a,key,index ) <==> pre_gen(a,key,index )
{
}

predicate post_original(a: array<int>,key: int,index: int)
  requires pre_original(a,key,index)
  reads a
{
  ( -1<=index<a.Length) && ( index!=-1 ==> a[index]==key && (forall i :: 0 <= i < index ==> a[i] != key))
  && ( index == -1 ==> (forall i::0 <= i < a.Length ==> a[i] != key))
}

predicate post_gen(a: array<int>,key: int,index: int)
  requires pre_original(a,key,index)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: array<int>,key: int,index: int)
  requires pre_original(a,key,index )
  requires pre_gen(a,key,index )
  ensures post_original(a,key,index ) <==> post_gen(a,key,index )
{
}

