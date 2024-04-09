predicate pre_original<T(==)>(s: set<T>,xs: seq<T>){
  true
}

predicate pre_gen<T(==)>(s: set<T>,xs: seq<T>)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq<T(==)>(s: set<T>,xs: seq<T>)
  ensures pre_original(s,xs ) <==> pre_gen(s,xs )
{
}

predicate post_original<T(==)>(s: set<T>,xs: seq<T>)
  requires pre_original(s,xs){
  ( |s| == |xs|)
}

predicate post_gen<T(==)>(s: set<T>,xs: seq<T>)
  requires pre_original(s,xs){
  true // (#POST) && ... (#POST)
}

lemma post_eq<T(==)>(s: set<T>,xs: seq<T>)
  requires pre_original(s,xs )
  requires pre_gen(s,xs )
  ensures post_original(s,xs ) <==> post_gen(s,xs )
{
}

