method Triple(x:int) returns (r:int)
  ensures r == 3 * x
{
  var count := 0;
  var i := 0;
  while (count < 3)
    invariant 0 <= count <= 3;
    invariant r == count * x;
    invariant i <= count;
    invariant count <= i + 1;
    decreases 3 - count;
  {
    count := count + 1;
    i := i + 1;
    r := r + x;
  }
  return r;
}
