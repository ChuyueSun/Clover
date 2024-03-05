method below_zero(operations: seq<int>) returns (s:array<int>, result:bool)
  ensures s.Length == |operations| + 1
  ensures s[0]==0
  ensures forall i :: 0 <= i < s.Length-1 ==> s[i+1]==s[i]+operations[i]
  ensures result == true ==> (exists i :: 1 <= i <= |operations| && s[i] < 0)
  ensures result == false ==> forall i :: 0 <= i < s.Length ==> s[i] >= 0
{
  result := false;
  s := new int[|operations| + 1];
  var i := 0;
  s[i] := 0;
  while i < s.Length
    invariant 0 <= i <= s.Length
    invariant s[0]==0
    invariant s.Length == |operations| + 1
    invariant forall x :: 0 <= x < i-1 ==> s[x+1]==s[x]+operations[x]
  {
    if i>0{
      s[i] := s[i - 1] + operations[i - 1];
    }
    i := i + 1;
  }
  i:=0;
  while i < s.Length
    invariant 0 <= i <= s.Length
    invariant forall x :: 0 <= x < i ==> s[x] >= 0
  {
    if s[i] < 0 {
      result := true;
      return;
    }
    i := i + 1;
  }
}
