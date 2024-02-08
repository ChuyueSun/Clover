predicate pre_original(src: array<int>, sStart: nat, dest: array<int>, dStart: nat, len: nat, r:array<int>)
  reads src, dest
{
  (src.Length >= sStart + len) && (dest.Length >= dStart + len)
}

predicate pre_gen(src: array<int>, sStart: nat, dest: array<int>, dStart: nat, len: nat, r:array<int>)
  reads src, dest
{
  true // (#PRE) && ... (#PRE)
}

lemma pre_eq(src: array<int>, sStart: nat, dest: array<int>, dStart: nat, len: nat, r:array<int>)
  ensures pre_original(src, sStart, dest, dStart, len, r) <==> pre_gen(src, sStart, dest, dStart, len, r)
{
}

predicate post_original(src: array<int>, sStart: nat, dest: array<int>, dStart: nat, len: nat, r:array<int>)
  requires pre_original(src, sStart, dest, dStart, len, r)
  reads src, dest, r
{
  (r.Length == dest.Length) &&
  (r[..dStart] == dest[..dStart]) &&
  (r[dStart + len..] == dest[dStart + len..]) &&
  (r[dStart..len+dStart] == src[sStart..len+sStart])
}

predicate post_gen(src: array<int>, sStart: nat, dest: array<int>, dStart: nat, len: nat, r:array<int>)
  requires pre_original(src, sStart, dest, dStart, len, r)
  reads src, dest, r
{
  true // (#POST) && ... (#POST)
}

lemma post_eq(src: array<int>, sStart: nat, dest: array<int>, dStart: nat, len: nat, r:array<int>)
  requires pre_original(src, sStart, dest, dStart, len, r)
  requires pre_gen(src, sStart, dest, dStart, len, r)
  ensures post_original(src, sStart, dest, dStart, len, r) <==> post_gen(src, sStart, dest, dStart, len, r)
{
}

