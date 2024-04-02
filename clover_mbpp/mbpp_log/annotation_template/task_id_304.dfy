twostate predicate pre_original(l: seq<int>, n: int, index: int, element: int)
    {
    true
    }

    twostate predicate pre_gen(l: seq<int>, n: int, index: int, element: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(l: seq<int>, n: int, index: int, element: int)
    ensures pre_original(l, n, index, element) <==> pre_gen(l, n, index, element)
    {
    }

    twostate predicate post_original(l: seq<int>, n: int, index: int, element: int)
    requires pre_original(l, n, index, element)
    {
    true
    }

    twostate predicate post_gen(l: seq<int>, n: int, index: int, element: int)
    requires pre_original(l, n, index, element)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(l: seq<int>, n: int, index: int, element: int)
    requires pre_original(l, n, index, element)
    requires pre_gen(l, n, index, element)
    ensures post_original(l, n, index, element) <==> post_gen(l, n, index, element)
    {
    }