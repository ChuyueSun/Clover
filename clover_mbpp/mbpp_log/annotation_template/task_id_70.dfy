twostate predicate pre_original(sequences: seq<seq<int>>, result: bool)
    {
    true
    }

    twostate predicate pre_gen(sequences: seq<seq<int>>, result: bool)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(sequences: seq<seq<int>>, result: bool)
    ensures pre_original(sequences, result) <==> pre_gen(sequences, result)
    {
    }

    twostate predicate post_original(sequences: seq<seq<int>>, result: bool)
    requires pre_original(sequences, result)
    {
    true
    }

    twostate predicate post_gen(sequences: seq<seq<int>>, result: bool)
    requires pre_original(sequences, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(sequences: seq<seq<int>>, result: bool)
    requires pre_original(sequences, result)
    requires pre_gen(sequences, result)
    ensures post_original(sequences, result) <==> post_gen(sequences, result)
    {
    }