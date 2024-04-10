method copy( src: array<int>, sStart: nat, dest: array<int>, dStart: nat, len: nat) returns (r: array<int>)
  requires src.Length >= sStart + len
  requires dest.Length >= dStart + len
  ensures r.Length == dest.Length
  ensures r[..dStart] == dest[..dStart]
  ensures r[dStart + len..] == dest[dStart + len..]
  ensures r[dStart..len+dStart] == src[sStart..len+sStart]

{
  if len == 0 { return dest; }
  var i: nat := 0;
  r := new int[dest.Length];
  while (i < r.Length)
    invariant i <= r.Length
    invariant r[..i] == dest[..i]
  {
    r[i] := dest[i];
    i := i + 1;
  }
  assert r[..]==dest[..];
  i := 0;
  while (i < len)
    invariant i <= len
    invariant r[..dStart] == dest[..dStart]
    invariant r[(dStart + len)..] == dest[(dStart + len)..]
    invariant r[dStart .. dStart + i] == src[sStart .. sStart + i]
  {
    assert r[(dStart + len)..] == dest[(dStart + len)..];
    r[dStart + i] := src[sStart + i];
    i := i + 1;
  }
}