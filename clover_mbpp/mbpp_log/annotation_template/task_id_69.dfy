twostate predicate pre_original(list: seq<seq<int>>, sub: seq<int>, result: bool)
    {
    true
    }

    twostate predicate pre_gen(list: seq<seq<int>>, sub: seq<int>, result: bool)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(list: seq<seq<int>>, sub: seq<int>, result: bool)
    ensures pre_original(list, sub, result) <==> pre_gen(list, sub, result)
    {
    }

    twostate predicate post_original(list: seq<seq<int>>, sub: seq<int>, result: bool)
    requires pre_original(list, sub, result)
    {
    true
    }

    twostate predicate post_gen(list: seq<seq<int>>, sub: seq<int>, result: bool)
    requires pre_original(list, sub, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(list: seq<seq<int>>, sub: seq<int>, result: bool)
    requires pre_original(list, sub, result)
    requires pre_gen(list, sub, result)
    ensures post_original(list, sub, result) <==> post_gen(list, sub, result)
    {
    }