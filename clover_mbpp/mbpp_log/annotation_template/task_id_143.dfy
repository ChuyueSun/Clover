twostate predicate pre_original(arrays: seq<array<int>>, count: int)
    reads arrays
    {
    true
    }

    twostate predicate pre_gen(arrays: seq<array<int>>, count: int)
    reads arrays

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(arrays: seq<array<int>>, count: int)
    ensures pre_original(arrays, count) <==> pre_gen(arrays, count)
    {
    }

    twostate predicate post_original(arrays: seq<array<int>>, count: int)
    reads arrays
    requires pre_original(arrays, count)
    {
    true
    }

    twostate predicate post_gen(arrays: seq<array<int>>, count: int)
    reads arrays
    requires pre_original(arrays, count)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(arrays: seq<array<int>>, count: int)
    requires pre_original(arrays, count)
    requires pre_gen(arrays, count)
    ensures post_original(arrays, count) <==> post_gen(arrays, count)
    {
    }