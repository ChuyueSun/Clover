method iter_copy<T(0)>(s: array<T>) returns (t: array<T>)
  //TOFILL
{
  t := new T[s.Length];
  var i:= 0;
  while (i < s.Length)
    invariant 0 <= i <= s.Length
    invariant forall x :: 0 <= x < i ==> s[x] == t[x]
  {
    t[i] := s[i];
    i:=i+1;
  }
}
