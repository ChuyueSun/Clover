method copy( src: array<int>, sStart: nat, dest: array<int>, dStart: nat, len: nat) returns (r: array<int>)
  requires src.Length >= sStart + len
  requires dest.Length >= dStart + len
  ensures r.Length == src.Length
  ensures (forall i :: 0 <= i < r.Length ==>
                         if sStart <= i < sStart + len then r[i] == dest[i - sStart + dStart] else r[i] == src[i])

{
  if len == 0 { return src; }
  var i: nat := 0;
  r := new int[src.Length];
  while (i < r.Length)
    invariant i <= r.Length
    invariant r[..i] == src[..i]
  {
    r[i] := src[i];
    i := i + 1;
  }

  i := 0;
  while (i < len)
    invariant i <= len
    invariant r[..sStart] == src[..sStart]
    invariant r[(sStart + len)..] == src[(sStart + len)..]
    invariant r[sStart .. sStart + i] == dest[dStart .. dStart + i]
  {
    r[sStart + i] := dest[dStart + i];
    i := i + 1;
  }
}
