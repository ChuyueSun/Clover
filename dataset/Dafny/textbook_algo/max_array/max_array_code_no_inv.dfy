method maxArray(a: array<int>) returns (m: int)
{
  m := a[0];
  var index := 1;
  while (index < a.Length)
  {
    m := if m>a[index] then  m else a[index];
    index := index + 1;
  }
}
