predicate pre_original<T(==)>(s: array<T>,t: array<T>)
  reads s, t
{
  true
}

predicate pre_gen<T(==)>(s: array<T>,t: array<T>)
  reads s, t
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq<T(==)>(s: array<T>,t: array<T>)
  ensures pre_original(s,t ) <==> pre_gen(s,t )
{
}

predicate post_original<T(==)>(s: array<T>,t: array<T>)
  requires pre_original(s,t)
  reads s, t
{
  ( s.Length==t.Length) && ( forall i::0<=i<s.Length ==> s[i]==t[i])
}

predicate post_gen<T(==)>(s: array<T>,t: array<T>)
  requires pre_original(s,t)
  reads s, t
{
  true // (#POST) && ... (#POST)
}

lemma post_eq<T(==)>(s: array<T>,t: array<T>)
  requires pre_original(s,t )
  requires pre_gen(s,t )
  ensures post_original(s,t ) <==> post_gen(s,t )
{
}

