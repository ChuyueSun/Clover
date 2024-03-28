predicate pre_original(lst: seq<int>, product : int)
{
true
}

predicate pre_gen(lst: seq<int>, product : int)
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(lst: seq<int>, product : int)
  ensures pre_original(lst, product) <==> pre_gen(lst, product)
{
}

predicate post_original(lst: seq<int>, product : int)
  requires pre_original(lst, product)
{
true
}

predicate post_gen(lst: seq<int>, product : int)
  requires pre_original(lst, product)
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(lst: seq<int>, product : int)
  requires pre_original(lst, product)
  requires pre_gen(lst, product)
  ensures post_original(lst, product) <==> post_gen(lst, product)
{
}

predicate IsEven(n: int)
{
  n % 2 == 0
}

predicate IsOdd(n: int)
{
  n % 2 != 0
}

predicate IsFirstEven(evenIndex: int, lst: seq<int>)
  requires 0 <= evenIndex < |lst|
  requires IsEven(lst[evenIndex])
{
  forall i :: 0 <= i < evenIndex ==> IsOdd(lst[i])
}

predicate IsFirstOdd(oddIndex: int, lst: seq<int>)
  requires 0 <= oddIndex < |lst|
  requires IsOdd(lst[oddIndex])
{
  forall i :: 0 <= i < oddIndex ==> IsEven(lst[i])
}


