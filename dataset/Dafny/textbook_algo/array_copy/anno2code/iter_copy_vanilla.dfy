method iter_copy<T>(s: array<T>) returns (t: array<T>)
  ensures s.Length == t.Length
  ensures forall i :: 0 <= i < s.Length ==> s[i] == t[i]
{
  var n := s.Length;
  var t := new T[n];
  var i := 0;
  while i < n
    decreases n - i
    invariant 0 <= i <= n
    invariant t.Length == n
    invariant forall j :: 0 <= j < i ==> s[j] == t[j]
  {
    t[i] := s[i];
    i := i + 1;
  }
  return t;
}
