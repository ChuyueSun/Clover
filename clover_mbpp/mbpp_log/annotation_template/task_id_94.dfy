twostate predicate pre_original(s: array<seq<int>>, firstOfMinSecond: int)
    reads s
    {
    true
    }

    twostate predicate pre_gen(s: array<seq<int>>, firstOfMinSecond: int)
    reads s

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s: array<seq<int>>, firstOfMinSecond: int)
    ensures pre_original(s, firstOfMinSecond) <==> pre_gen(s, firstOfMinSecond)
    {
    }

    twostate predicate post_original(s: array<seq<int>>, firstOfMinSecond: int)
    reads s
    requires pre_original(s, firstOfMinSecond)
    {
    true
    }

    twostate predicate post_gen(s: array<seq<int>>, firstOfMinSecond: int)
    reads s
    requires pre_original(s, firstOfMinSecond)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s: array<seq<int>>, firstOfMinSecond: int)
    requires pre_original(s, firstOfMinSecond)
    requires pre_gen(s, firstOfMinSecond)
    ensures post_original(s, firstOfMinSecond) <==> post_gen(s, firstOfMinSecond)
    {
    }