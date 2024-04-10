method LinearSearch3<T>(a: array<T>, P: T -> bool) returns (n: int)

{
  assert exists i :: 0 <= i < a.Length && P(a[i]);

  n := 0;
  while true
    invariant 0 <= n < a.Length
    invariant exists i :: n <= i < a.Length && P(a[i])
    invariant  forall k :: 0 <= k < n ==> !P(a[k])
    decreases a.Length - n
  {
    if P(a[n]) {
      return;
    }
    n := n + 1;
  }
}
