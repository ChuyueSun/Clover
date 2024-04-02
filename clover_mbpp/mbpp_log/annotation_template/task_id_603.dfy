twostate predicate pre_original(n: int, lucid: seq<int>)
    {
    true
    }

    twostate predicate pre_gen(n: int, lucid: seq<int>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(n: int, lucid: seq<int>)
    ensures pre_original(n, lucid) <==> pre_gen(n, lucid)
    {
    }

    twostate predicate post_original(n: int, lucid: seq<int>)
    requires pre_original(n, lucid)
    {
    true
    }

    twostate predicate post_gen(n: int, lucid: seq<int>)
    requires pre_original(n, lucid)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(n: int, lucid: seq<int>)
    requires pre_original(n, lucid)
    requires pre_gen(n, lucid)
    ensures post_original(n, lucid) <==> post_gen(n, lucid)
    {
    }