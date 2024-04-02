twostate predicate pre_original(lst: seq<seq<int>>, result: seq<int>)
    {
    true
    }

    twostate predicate pre_gen(lst: seq<seq<int>>, result: seq<int>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(lst: seq<seq<int>>, result: seq<int>)
    ensures pre_original(lst, result) <==> pre_gen(lst, result)
    {
    }

    twostate predicate post_original(lst: seq<seq<int>>, result: seq<int>)
    requires pre_original(lst, result)
    {
    true
    }

    twostate predicate post_gen(lst: seq<seq<int>>, result: seq<int>)
    requires pre_original(lst, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(lst: seq<seq<int>>, result: seq<int>)
    requires pre_original(lst, result)
    requires pre_gen(lst, result)
    ensures post_original(lst, result) <==> post_gen(lst, result)
    {
    }