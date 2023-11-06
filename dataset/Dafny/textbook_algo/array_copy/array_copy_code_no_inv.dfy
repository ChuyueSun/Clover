method iter_copy<T(0)>(s: array<T>) returns (t: array<T>)
{
  t := new T[s.Length];
  var i:= 0;
  while (i < s.Length)
  {
    t[i] := s[i];
    i:=i+1;
  }

}
