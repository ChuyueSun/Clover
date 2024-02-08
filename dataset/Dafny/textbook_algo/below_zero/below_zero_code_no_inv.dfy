method below_zero(operations: seq<int>) returns (s:array<int>, result:bool)
{
  result := false;
  s := new int[|operations| + 1];
  var i := 0;
  s[i] := 0;
  while i < s.Length
  {
    if i>0{
        s[i] := s[i - 1] + operations[i - 1];
    }
    i := i + 1;
  }
  i:=0;
  while i < s.Length
  {
    if s[i] < 0 {
      result := true;
      return;
    }
    i := i + 1;
  }
}
