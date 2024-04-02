twostate predicate pre_original(s: seq<int>, k: int, result: bool)
    {
    true
    }

    twostate predicate pre_gen(s: seq<int>, k: int, result: bool)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s: seq<int>, k: int, result: bool)
    ensures pre_original(s, k, result) <==> pre_gen(s, k, result)
    {
    }

    twostate predicate post_original(s: seq<int>, k: int, result: bool)
    requires pre_original(s, k, result)
    {
    true
    }

    twostate predicate post_gen(s: seq<int>, k: int, result: bool)
    requires pre_original(s, k, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s: seq<int>, k: int, result: bool)
    requires pre_original(s, k, result)
    requires pre_gen(s, k, result)
    ensures post_original(s, k, result) <==> post_gen(s, k, result)
    {
    }