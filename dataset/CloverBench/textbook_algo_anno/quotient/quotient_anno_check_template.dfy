predicate pre_original(x: int,y:int,r:int,q:int){
  y != 0
}

predicate pre_gen(x: int,y:int,r:int,q:int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(x: int,y:int,r:int,q:int)
  ensures pre_original(x,y,r,q ) <==> pre_gen(x,y,r,q )
{
}

predicate post_original(x: int,y:int,r:int,q:int)
  requires pre_original(x,y,r,q){
  ( q*y+r==x) && ( y>r>=0) && (q>=0)
}

predicate post_gen(x: int,y:int,r:int,q:int)
  requires pre_original(x,y,r,q){
  true // (#POST) && ... (#POST)
}

lemma post_eq(x: int,y:int,r:int,q:int)
  requires pre_original(x,y,r,q )
  requires pre_gen(x,y,r,q )
  ensures post_original(x,y,r,q ) <==> post_gen(x,y,r,q )
{
}

