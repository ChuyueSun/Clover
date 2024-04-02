twostate predicate pre_original(a: seq<int>, b: seq<int>, diff: seq<int>)
    {
    true
    }

    twostate predicate pre_gen(a: seq<int>, b: seq<int>, diff: seq<int>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: seq<int>, b: seq<int>, diff: seq<int>)
    ensures pre_original(a, b, diff) <==> pre_gen(a, b, diff)
    {
    }

    twostate predicate post_original(a: seq<int>, b: seq<int>, diff: seq<int>)
    requires pre_original(a, b, diff)
    {
    true
    }

    twostate predicate post_gen(a: seq<int>, b: seq<int>, diff: seq<int>)
    requires pre_original(a, b, diff)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: seq<int>, b: seq<int>, diff: seq<int>)
    requires pre_original(a, b, diff)
    requires pre_gen(a, b, diff)
    ensures post_original(a, b, diff) <==> post_gen(a, b, diff)
    {
    }