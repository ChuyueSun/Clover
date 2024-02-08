predicate pre_original(x: int,y:int,z: int){
  true
}

predicate pre_gen(x: int,y:int,z: int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(x: int,y:int,z: int)
  ensures pre_original(x,y,z ) <==> pre_gen(x,y,z )
{
}

predicate post_original(x: int,y:int,z: int)
  requires pre_original(x,y,z){
  ( x<=y ==> z==x) && ( x>y ==> z==y)
}

predicate post_gen(x: int,y:int,z: int)
  requires pre_original(x,y,z){
  true // (#POST) && ... (#POST)
}

lemma post_eq(x: int,y:int,z: int)
  requires pre_original(x,y,z )
  requires pre_gen(x,y,z )
  ensures post_original(x,y,z ) <==> post_gen(x,y,z )
{
}

