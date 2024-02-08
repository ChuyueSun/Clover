predicate pre_original(x: int,y: int){
  true
}

predicate pre_gen(x: int,y: int){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(x: int,y: int)
  ensures pre_original(x,y ) <==> pre_gen(x,y )
{
}

predicate post_original(x: int,y: int)
  requires pre_original(x,y){
  x>=0 ==> x==y
}

predicate post_gen(x: int,y: int)
  requires pre_original(x,y){
  true // (#POST) && ... (#POST)
}

lemma post_eq(x: int,y: int)
  requires pre_original(x,y )
  requires pre_gen(x,y )
  ensures post_original(x,y ) <==> post_gen(x,y )
{
}

