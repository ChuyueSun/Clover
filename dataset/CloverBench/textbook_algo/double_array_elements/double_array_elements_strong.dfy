method double_array_elements(s: array<int>)
  modifies s
  ensures forall i :: 0 <= i < s.Length ==> s[i] == 2 * old(s[i])
{
  var i:= 0;
  while (i < s.Length)
    invariant 0 <= i <= s.Length
    invariant forall x :: i <= x < s.Length ==> s[x] == old(s[x])
    invariant forall x :: 0 <= x < i ==> s[x] == 2 * old(s[x])

  {
    s[i] :=  2 * s[i];
    i := i + 1;
  }
}