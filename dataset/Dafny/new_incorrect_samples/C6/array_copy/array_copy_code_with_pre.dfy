method iter_copy<T(0)>(s: array<T>) returns (t: array<T>)
  ensures s.Length==t.Length
{
  t := new T[s.Length];
  var i:= 0;
  while (i < s.Length)
    invariant 0 <= i <= s.Length
  {
    t[i] := s[s.Length - 1 - i];
    i:=i+1;
  }
}
