twostate predicate pre_original(s: seq<int>, a: array<int>, r: seq<int>)
    reads a
    {
    true
    }

    twostate predicate pre_gen(s: seq<int>, a: array<int>, r: seq<int>)
    reads a

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s: seq<int>, a: array<int>, r: seq<int>)
    ensures pre_original(s, a, r) <==> pre_gen(s, a, r)
    {
    }

    twostate predicate post_original(s: seq<int>, a: array<int>, r: seq<int>)
    reads a
    requires pre_original(s, a, r)
    {
    true
    }

    twostate predicate post_gen(s: seq<int>, a: array<int>, r: seq<int>)
    reads a
    requires pre_original(s, a, r)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s: seq<int>, a: array<int>, r: seq<int>)
    requires pre_original(s, a, r)
    requires pre_gen(s, a, r)
    ensures post_original(s, a, r) <==> post_gen(s, a, r)
    {
    }