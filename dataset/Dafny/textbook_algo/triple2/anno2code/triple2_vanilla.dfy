method Triple (x:int) returns (r:int)
  ensures r==3*x
{
  var i:int := 0;
  while (i < 3)
    invariant i <= 3
    invariant r == i * x
    decreases 3 - i
  {
    r := r + x;
    i := i + 1;
  }
}
