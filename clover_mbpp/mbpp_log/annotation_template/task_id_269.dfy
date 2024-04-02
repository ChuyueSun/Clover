twostate predicate pre_original(c: char, ascii: int)
    {
    true
    }

    twostate predicate pre_gen(c: char, ascii: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(c: char, ascii: int)
    ensures pre_original(c, ascii) <==> pre_gen(c, ascii)
    {
    }

    twostate predicate post_original(c: char, ascii: int)
    requires pre_original(c, ascii)
    {
    true
    }

    twostate predicate post_gen(c: char, ascii: int)
    requires pre_original(c, ascii)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(c: char, ascii: int)
    requires pre_original(c, ascii)
    requires pre_gen(c, ascii)
    ensures post_original(c, ascii) <==> post_gen(c, ascii)
    {
    }