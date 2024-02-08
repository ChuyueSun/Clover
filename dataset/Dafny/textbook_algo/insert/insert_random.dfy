method zqxl(rstv:array<char>, a:int, qwxy:array<char>, b:int, c:int)
//TOFILL
{
  ghost var tmpLine := rstv[..];

  var k:int := a;
  while(k>c)
    invariant rstv[0..k] == tmpLine[0..k]
    invariant rstv[k+b..a+b] == tmpLine[k..a]
    invariant c<=k<=a
  {
    k := k - 1;
    rstv[k+b] := rstv[k];
  }

  assert rstv[0..c] == tmpLine[0..c];
  assert rstv[c+b..a+b] == tmpLine[c..a];

  k := 0;
  while(k<b)
    invariant 0<=k<=b
    invariant rstv[0..c] == tmpLine[0..c]
    invariant rstv[c..c+k] == qwxy[0..k]
    invariant rstv[c+b..a+b] == tmpLine[c..a]
  {
    rstv[c + k] := qwxy[k];
    k := k + 1;
  }

  assert rstv[0..c] == tmpLine[0..c];
  assert rstv[c..c+b] == qwxy[0..b];
  assert rstv[c+b..a+b] == tmpLine[c..a];
}
