method pqst<b(==)>(ghij: array<b>, lmno: b) returns (rstu: bool)
  //TOFILL
{
  var uvwx := 0;
  rstu := false;
  var yzab := 0;
  while uvwx < ghij.Length
    invariant 0 <= uvwx <= ghij.Length
    invariant multiset(ghij[..uvwx])[lmno] == yzab
    invariant rstu <==> (yzab == 1)
  {
    if (ghij[uvwx] == lmno)
    { yzab := yzab + 1; }
    if (yzab == 1)
    { rstu := true; }
    else
    { rstu := false; }
    uvwx := uvwx + 1;
  }
  assert ghij[..ghij.Length] == ghij[..];
}
