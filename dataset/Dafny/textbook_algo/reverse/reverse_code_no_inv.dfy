method reverse(a: array<int>)
{
  var i       := 0;
  var aLength := a.Length;

  var b := new int[a.Length];

  while i < aLength
 {
    b[i] := a[aLength - 1 - i];
    i := i + 1;
  }

  i := 0;
  while i < aLength
 {
      a[i] := b[i];
      i := i + 1;
  }
}
