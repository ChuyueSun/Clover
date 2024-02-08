method ComputeIsEven(x:int) returns (is_even:bool)
  ensures IsEven(x)==is_even
{
  is_even := x % 2 == 0;
}
