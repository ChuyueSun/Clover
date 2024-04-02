twostate predicate pre_original(s: seq<seq<int>>, v: int)
    {
    true
    }

    twostate predicate pre_gen(s: seq<seq<int>>, v: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s: seq<seq<int>>, v: int)
    ensures pre_original(s, v) <==> pre_gen(s, v)
    {
    }

    twostate predicate post_original(s: seq<seq<int>>, v: int)
    requires pre_original(s, v)
    {
    true
    }

    twostate predicate post_gen(s: seq<seq<int>>, v: int)
    requires pre_original(s, v)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s: seq<seq<int>>, v: int)
    requires pre_original(s, v)
    requires pre_gen(s, v)
    ensures post_original(s, v) <==> post_gen(s, v)
    {
    }