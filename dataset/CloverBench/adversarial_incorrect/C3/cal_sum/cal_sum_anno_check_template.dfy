predicate pre_original(s:int,n:int){
  true
}

predicate pre_gen(s:int,n:int){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(s:int,n:int)
  ensures pre_original(s,n ) <==> pre_gen(s,n )
{
}

predicate post_original(s:int,n:int)
  requires pre_original(s,n){
  ( s == n * (n + 1) / 2)
}

predicate post_gen(s:int,n:int)
  requires pre_original(s,n){
  true // (#POST) && ... (#POST)
}

lemma post_eq(s:int,n:int)
  requires pre_original(s,n )
  requires pre_gen(s,n )
  ensures post_original(s,n ) <==> post_gen(s,n )
{
}

