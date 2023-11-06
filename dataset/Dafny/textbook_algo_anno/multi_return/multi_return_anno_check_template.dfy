predicate pre_original(x: int,y: int,more: int,less: int){
  true
}

predicate pre_gen(x: int,y: int,more: int,less: int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(x: int,y: int,more: int,less: int)
  ensures pre_original(x,y,more,less ) <==> pre_gen(x,y,more,less )
{
}

predicate post_original(x: int,y: int,more: int,less: int)
  requires pre_original(x,y,more,less){
  ( more == x+y) && ( less == x-y)
}

predicate post_gen(x: int,y: int,more: int,less: int)
  requires pre_original(x,y,more,less){
  true // (#POST) && ... (#POST)
}

lemma post_eq(x: int,y: int,more: int,less: int)
  requires pre_original(x,y,more,less )
  requires pre_gen(x,y,more,less )
  ensures post_original(x,y,more,less ) <==> post_gen(x,y,more,less )
{
}

