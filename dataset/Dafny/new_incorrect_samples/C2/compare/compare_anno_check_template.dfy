predicate pre_original<T(==)>(a: T,b: T,eq: bool){
  a==b
}

predicate pre_gen<T(==)>(a: T,b: T,eq: bool){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq<T(==)>(a: T,b: T,eq: bool)
  ensures pre_original(a,b,eq ) <==> pre_gen(a,b,eq )
{
}

predicate post_original<T(==)>(a: T,b: T,eq: bool)
  requires pre_original(a,b,eq){
  ( a==b ==> eq==true) && ( a!=b ==> eq==false)
}

predicate post_gen<T(==)>(a: T,b: T,eq: bool)
  requires pre_original(a,b,eq){
  true // (#POST) && ... (#POST)
}

lemma post_eq<T(==)>(a: T,b: T,eq: bool)
  requires pre_original(a,b,eq )
  requires pre_gen(a,b,eq )
  ensures post_original(a,b,eq ) <==> post_gen(a,b,eq )
{
}

