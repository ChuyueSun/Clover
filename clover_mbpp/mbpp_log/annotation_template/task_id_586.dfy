twostate predicate pre_original(l: seq<int>, n: int, r: seq<int>)
    {
    true
    }

    twostate predicate pre_gen(l: seq<int>, n: int, r: seq<int>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(l: seq<int>, n: int, r: seq<int>)
    ensures pre_original(l, n, r) <==> pre_gen(l, n, r)
    {
    }

    twostate predicate post_original(l: seq<int>, n: int, r: seq<int>)
    requires pre_original(l, n, r)
    {
    true
    }

    twostate predicate post_gen(l: seq<int>, n: int, r: seq<int>)
    requires pre_original(l, n, r)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(l: seq<int>, n: int, r: seq<int>)
    requires pre_original(l, n, r)
    requires pre_gen(l, n, r)
    ensures post_original(l, n, r) <==> post_gen(l, n, r)
    {
    }