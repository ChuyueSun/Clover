twostate predicate pre_original(s: seq<int>, copy: seq<int>)
    {
    true
    }

    twostate predicate pre_gen(s: seq<int>, copy: seq<int>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(s: seq<int>, copy: seq<int>)
    ensures pre_original(s, copy) <==> pre_gen(s, copy)
    {
    }

    twostate predicate post_original(s: seq<int>, copy: seq<int>)
    requires pre_original(s, copy)
    {
    true
    }

    twostate predicate post_gen(s: seq<int>, copy: seq<int>)
    requires pre_original(s, copy)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(s: seq<int>, copy: seq<int>)
    requires pre_original(s, copy)
    requires pre_gen(s, copy)
    ensures post_original(s, copy) <==> post_gen(s, copy)
    {
    }