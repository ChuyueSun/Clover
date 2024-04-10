predicate pre_original(x: seq<char>, result: bool){
  true
}

predicate pre_gen(x: seq<char>, result: bool){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(x: seq<char>, result: bool)
  ensures pre_original(x, result) <==> pre_gen(x, result)
{
}

predicate post_original(x: seq<char>, result: bool)
  requires pre_original(x,result){
  result ==> (forall i :: 0 <= i < |x| ==> x[i] == x[|x| - i - 1])
}

predicate post_gen(x: seq<char>, result: bool)
  requires pre_original(x,result){
  true // (#POST) && ... (#POST)
}

lemma post_eq(x: seq<char>, result: bool)
  requires pre_original(x, result)
  requires pre_gen(x, result)
  ensures post_original(x, result) <==> post_gen(x, result)
{
}

