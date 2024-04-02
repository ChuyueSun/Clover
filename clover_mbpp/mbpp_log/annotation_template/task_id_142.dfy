twostate predicate pre_original(a: seq<int>, b: seq<int>, c: seq<int>, count: int)
    {
    true
    }

    twostate predicate pre_gen(a: seq<int>, b: seq<int>, c: seq<int>, count: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: seq<int>, b: seq<int>, c: seq<int>, count: int)
    ensures pre_original(a, b, c, count) <==> pre_gen(a, b, c, count)
    {
    }

    twostate predicate post_original(a: seq<int>, b: seq<int>, c: seq<int>, count: int)
    requires pre_original(a, b, c, count)
    {
    true
    }

    twostate predicate post_gen(a: seq<int>, b: seq<int>, c: seq<int>, count: int)
    requires pre_original(a, b, c, count)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: seq<int>, b: seq<int>, c: seq<int>, count: int)
    requires pre_original(a, b, c, count)
    requires pre_gen(a, b, c, count)
    ensures post_original(a, b, c, count) <==> post_gen(a, b, c, count)
    {
    }