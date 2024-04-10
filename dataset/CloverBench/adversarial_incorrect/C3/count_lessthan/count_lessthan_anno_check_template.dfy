predicate pre_original(numbers: set<int>,threshold: int,count: int){
  true
}

predicate pre_gen(numbers: set<int>,threshold: int,count: int){
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(numbers: set<int>,threshold: int,count: int)
  ensures pre_original(numbers,threshold,count ) <==> pre_gen(numbers,threshold,count )
{
}

predicate post_original(numbers: set<int>,threshold: int,count: int)
  requires pre_original(numbers,threshold,count){
  0 <= count <= |numbers|
}

predicate post_gen(numbers: set<int>,threshold: int,count: int)
  requires pre_original(numbers,threshold,count){
  true // (#POST) && ... (#POST)
}

lemma post_eq(numbers: set<int>,threshold: int,count: int)
  requires pre_original(numbers,threshold,count )
  requires pre_gen(numbers,threshold,count )
  ensures post_original(numbers,threshold,count ) <==> post_gen(numbers,threshold,count )
{
}

