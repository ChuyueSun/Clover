predicate pre_original(x:int,is_even:bool){
  true
}

predicate pre_gen(x:int,is_even:bool){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(x:int,is_even:bool)
  ensures pre_original(x,is_even ) <==> pre_gen(x,is_even )
{
}

predicate post_original(x:int,is_even:bool)
  requires pre_original(x,is_even){
  ( (x % 2 == 0)==is_even)
}

predicate post_gen(x:int,is_even:bool)
  requires pre_original(x,is_even){
  true // (#POST) && ... (#POST)
}

lemma post_eq(x:int,is_even:bool)
  requires pre_original(x,is_even )
  requires pre_gen(x,is_even )
  ensures post_original(x,is_even ) <==> post_gen(x,is_even )
{
}

