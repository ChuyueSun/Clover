twostate predicate pre_original(a: seq<int>, b: seq<int>, result: bool)
    {
    true
    }

    twostate predicate pre_gen(a: seq<int>, b: seq<int>, result: bool)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: seq<int>, b: seq<int>, result: bool)
    ensures pre_original(a, b, result) <==> pre_gen(a, b, result)
    {
    }

    twostate predicate post_original(a: seq<int>, b: seq<int>, result: bool)
    requires pre_original(a, b, result)
    {
    true
    }

    twostate predicate post_gen(a: seq<int>, b: seq<int>, result: bool)
    requires pre_original(a, b, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: seq<int>, b: seq<int>, result: bool)
    requires pre_original(a, b, result)
    requires pre_gen(a, b, result)
    ensures post_original(a, b, result) <==> post_gen(a, b, result)
    {
    }