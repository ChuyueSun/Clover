predicate pre_original(x: int,seven: int){
  true
}

predicate pre_gen(x: int,seven: int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(x: int,seven: int)
  ensures pre_original(x,seven ) <==> pre_gen(x,seven )
{
}

predicate post_original(x: int,seven: int)
  requires pre_original(x,seven){
  true
}

predicate post_gen(x: int,seven: int)
  requires pre_original(x,seven){
  true // (#POST) && ... (#POST)
}

lemma post_eq(x: int,seven: int)
  requires pre_original(x,seven )
  requires pre_gen(x,seven )
  ensures post_original(x,seven ) <==> post_gen(x,seven )
{
}

