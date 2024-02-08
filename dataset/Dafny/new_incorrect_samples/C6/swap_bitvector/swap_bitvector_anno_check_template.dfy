predicate pre_original(X: bv8,Y: bv8,x: bv8,y: bv8){
  true
}

predicate pre_gen(X: bv8,Y: bv8,x: bv8,y: bv8)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(X: bv8,Y: bv8,x: bv8,y: bv8)
  ensures pre_original(X,Y,x,y ) <==> pre_gen(X,Y,x,y )
{
}

predicate post_original(X: bv8,Y: bv8,x: bv8,y: bv8)
  requires pre_original(X,Y,x,y){
  ( x==Y) && ( y==X)
}

predicate post_gen(X: bv8,Y: bv8,x: bv8,y: bv8)
  requires pre_original(X,Y,x,y){
  true // (#POST) && ... (#POST)
}

lemma post_eq(X: bv8,Y: bv8,x: bv8,y: bv8)
  requires pre_original(X,Y,x,y )
  requires pre_gen(X,Y,x,y )
  ensures post_original(X,Y,x,y ) <==> post_gen(X,Y,x,y )
{
}

