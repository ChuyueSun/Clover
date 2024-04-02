twostate predicate pre_original(lists: seq<seq<int>>, maxList: seq<int>)
    {
    true
    }

    twostate predicate pre_gen(lists: seq<seq<int>>, maxList: seq<int>)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(lists: seq<seq<int>>, maxList: seq<int>)
    ensures pre_original(lists, maxList) <==> pre_gen(lists, maxList)
    {
    }

    twostate predicate post_original(lists: seq<seq<int>>, maxList: seq<int>)
    requires pre_original(lists, maxList)
    {
    true
    }

    twostate predicate post_gen(lists: seq<seq<int>>, maxList: seq<int>)
    requires pre_original(lists, maxList)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(lists: seq<seq<int>>, maxList: seq<int>)
    requires pre_original(lists, maxList)
    requires pre_gen(lists, maxList)
    ensures post_original(lists, maxList) <==> post_gen(lists, maxList)
    {
    }