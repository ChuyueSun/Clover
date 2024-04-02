twostate predicate pre_original(a: array<int>, s: seq<int>)
    reads a
    {
    true
    }

    twostate predicate pre_gen(a: array<int>, s: seq<int>)
    reads a

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: array<int>, s: seq<int>)
    ensures pre_original(a, s) <==> pre_gen(a, s)
    {
    }

    twostate predicate post_original(a: array<int>, s: seq<int>)
    reads a
    requires pre_original(a, s)
    {
    true
    }

    twostate predicate post_gen(a: array<int>, s: seq<int>)
    reads a
    requires pre_original(a, s)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: array<int>, s: seq<int>)
    requires pre_original(a, s)
    requires pre_gen(a, s)
    ensures post_original(a, s) <==> post_gen(a, s)
    {
    }