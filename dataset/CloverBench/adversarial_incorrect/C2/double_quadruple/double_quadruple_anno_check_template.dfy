predicate pre_original(x: int,a: int,b: int){
  x==0
}

predicate pre_gen(x: int,a: int,b: int){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(x: int,a: int,b: int)
  ensures pre_original(x,a,b ) <==> pre_gen(x,a,b )
{
}

predicate post_original(x: int,a: int,b: int)
  requires pre_original(x,a,b){
  ( a == 2 * x) && ( b == 4 * x) && (a==0)
}

predicate post_gen(x: int,a: int,b: int)
  requires pre_original(x,a,b){
  true // (#POST) && ... (#POST)
}

lemma post_eq(x: int,a: int,b: int)
  requires pre_original(x,a,b )
  requires pre_gen(x,a,b )
  ensures post_original(x,a,b ) <==> post_gen(x,a,b )
{
}

