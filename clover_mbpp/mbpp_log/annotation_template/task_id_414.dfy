twostate predicate pre_original(seq1: seq<int>, seq2: seq<int>, result: bool)
    {
    true
    }

    twostate predicate pre_gen(seq1: seq<int>, seq2: seq<int>, result: bool)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(seq1: seq<int>, seq2: seq<int>, result: bool)
    ensures pre_original(seq1, seq2, result) <==> pre_gen(seq1, seq2, result)
    {
    }

    twostate predicate post_original(seq1: seq<int>, seq2: seq<int>, result: bool)
    requires pre_original(seq1, seq2, result)
    {
    true
    }

    twostate predicate post_gen(seq1: seq<int>, seq2: seq<int>, result: bool)
    requires pre_original(seq1, seq2, result)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(seq1: seq<int>, seq2: seq<int>, result: bool)
    requires pre_original(seq1, seq2, result)
    requires pre_gen(seq1, seq2, result)
    ensures post_original(seq1, seq2, result) <==> post_gen(seq1, seq2, result)
    {
    }