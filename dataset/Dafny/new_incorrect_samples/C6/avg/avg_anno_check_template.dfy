predicate pre_original(a: int,b: int,avg:int){
  true
}

predicate pre_gen(a: int,b: int,avg:int){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(a: int,b: int,avg:int)
  ensures pre_original(a,b,avg ) <==> pre_gen(a,b,avg )
{
}

predicate post_original(a: int,b: int,avg:int)
  requires pre_original(a,b,avg){
  ( avg == (a+b)/2)
}

predicate post_gen(a: int,b: int,avg:int)
  requires pre_original(a,b,avg){
  true // (#POST) && ... (#POST)
}

lemma post_eq(a: int,b: int,avg:int)
  requires pre_original(a,b,avg )
  requires pre_gen(a,b,avg )
  ensures post_original(a,b,avg ) <==> post_gen(a,b,avg )
{
}

