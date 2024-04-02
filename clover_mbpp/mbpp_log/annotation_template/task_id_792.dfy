twostate predicate pre_original(lists: seq<seq<int>>, count: int)
    {
    true
    }

    twostate predicate pre_gen(lists: seq<seq<int>>, count: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(lists: seq<seq<int>>, count: int)
    ensures pre_original(lists, count) <==> pre_gen(lists, count)
    {
    }

    twostate predicate post_original(lists: seq<seq<int>>, count: int)
    requires pre_original(lists, count)
    {
    true
    }

    twostate predicate post_gen(lists: seq<seq<int>>, count: int)
    requires pre_original(lists, count)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(lists: seq<seq<int>>, count: int)
    requires pre_original(lists, count)
    requires pre_gen(lists, count)
    ensures post_original(lists, count) <==> post_gen(lists, count)
    {
    }