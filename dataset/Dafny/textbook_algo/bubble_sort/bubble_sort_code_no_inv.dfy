method BubbleSort(a: array<int>)
{
  var i := a.Length - 1;
  while (i > 0)

  {
    var j := 0;
    while (j < i)
{
      if (a[j] > a[j + 1])
      {
        a[j], a[j + 1] := a[j + 1], a[j];
      }
      j := j + 1;
    }
    i := i - 1;
  }

}
