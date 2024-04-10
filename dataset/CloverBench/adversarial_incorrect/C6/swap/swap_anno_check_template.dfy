predicate pre_original(X: int,Y: int,x: int,y: int){
  true
}

predicate pre_gen(X: int,Y: int,x: int,y: int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(X: int,Y: int,x: int,y: int)
  ensures pre_original(X,Y,x,y ) <==> pre_gen(X,Y,x,y )
{
}

predicate post_original(X: int,Y: int,x: int,y: int)
  requires pre_original(X,Y,x,y){
  ( x==Y) && ( y==X)
}

predicate post_gen(X: int,Y: int,x: int,y: int)
  requires pre_original(X,Y,x,y){
  true // (#POST) && ... (#POST)
}

lemma post_eq(X: int,Y: int,x: int,y: int)
  requires pre_original(X,Y,x,y )
  requires pre_gen(X,Y,x,y )
  ensures post_original(X,Y,x,y ) <==> post_gen(X,Y,x,y )
{
}

