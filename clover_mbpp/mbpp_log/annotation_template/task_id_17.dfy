twostate predicate pre_original(side: int, perimeter: int)
    {
    true
    }

    twostate predicate pre_gen(side: int, perimeter: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(side: int, perimeter: int)
    ensures pre_original(side, perimeter) <==> pre_gen(side, perimeter)
    {
    }

    twostate predicate post_original(side: int, perimeter: int)
    requires pre_original(side, perimeter)
    {
    true
    }

    twostate predicate post_gen(side: int, perimeter: int)
    requires pre_original(side, perimeter)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(side: int, perimeter: int)
    requires pre_original(side, perimeter)
    requires pre_gen(side, perimeter)
    ensures post_original(side, perimeter) <==> post_gen(side, perimeter)
    {
    }