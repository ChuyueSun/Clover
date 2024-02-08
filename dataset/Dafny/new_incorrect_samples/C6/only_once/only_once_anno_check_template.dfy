predicate pre_original<T(==)>(a: array<T>,key: T,b:bool)
  reads a
{
  true
}

predicate pre_gen<T(==)>(a: array<T>,key: T,b:bool)
  reads a
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq<T(==)>(a: array<T>,key: T,b:bool)
  ensures pre_original(a,key,b ) <==> pre_gen(a,key,b )
{
}

predicate post_original<T(==)>(a: array<T>,key: T,b:bool)
  requires pre_original(a,key,b)
  reads a
{
  ( (multiset(a[..])[key] ==1 ) <==> b)
}

predicate post_gen<T(==)>(a: array<T>,key: T,b:bool)
  requires pre_original(a,key,b)
  reads a
{
  true // (#POST) && ... (#POST)
}

lemma post_eq<T(==)>(a: array<T>,key: T,b:bool)
  requires pre_original(a,key,b )
  requires pre_gen(a,key,b )
  ensures post_original(a,key,b ) <==> post_gen(a,key,b )
{
}

