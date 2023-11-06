method ComputeAvg(a: int, b: int) returns (avg:int)
  requires 0 <= a && 0 <= b
  ensures avg == (a+b)/2
{
  var sum := a + b;
  avg := sum / 2;
}
