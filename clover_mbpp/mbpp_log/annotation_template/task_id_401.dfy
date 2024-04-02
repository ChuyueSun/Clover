twostate predicate pre_original(a: seq<seq<int>>, b: seq<seq<int>>, result: seq<seq<int>>)
    {
    true
    }

    twostate predicate pre_gen(a: seq<seq<int>>, b: seq<seq<int>>, result: seq<seq<int>>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(a: seq<seq<int>>, b: seq<seq<int>>, result: seq<seq<int>>)
    ensures pre_original(a, b, result) <==> pre_gen(a, b, result)
    {
    }

    twostate predicate post_original(a: seq<seq<int>>, b: seq<seq<int>>, result: seq<seq<int>>)
    requires pre_original(a, b, result)
    {
    true
    }

    twostate predicate post_gen(a: seq<seq<int>>, b: seq<seq<int>>, result: seq<seq<int>>)
    requires pre_original(a, b, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(a: seq<seq<int>>, b: seq<seq<int>>, result: seq<seq<int>>)
    requires pre_original(a, b, result)
    requires pre_gen(a, b, result)
    ensures post_original(a, b, result) <==> post_gen(a, b, result)
    {
    }