method foo(a: int, b: int) returns (avg:int)
  requires a == b
  ensures avg == a
{
  avg:= (a + b) / 2;
}
