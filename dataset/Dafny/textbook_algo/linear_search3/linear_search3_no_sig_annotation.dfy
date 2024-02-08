method foo<T>(a: array<T>, P: T -> bool) returns (n: int)
  requires exists i :: 0 <= i < a.Length && P(a[i])
  ensures 0 <= n < a.Length && P(a[n])
  ensures forall k :: 0 <= k < n ==> !P(a[k])
{
  //TOFILL
}
