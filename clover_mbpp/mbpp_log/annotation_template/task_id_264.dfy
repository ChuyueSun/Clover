twostate predicate pre_original(humanYears: int, dogYears: int)
    {
    true
    }

    twostate predicate pre_gen(humanYears: int, dogYears: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(humanYears: int, dogYears: int)
    ensures pre_original(humanYears, dogYears) <==> pre_gen(humanYears, dogYears)
    {
    }

    twostate predicate post_original(humanYears: int, dogYears: int)
    requires pre_original(humanYears, dogYears)
    {
    true
    }

    twostate predicate post_gen(humanYears: int, dogYears: int)
    requires pre_original(humanYears, dogYears)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(humanYears: int, dogYears: int)
    requires pre_original(humanYears, dogYears)
    requires pre_gen(humanYears, dogYears)
    ensures post_original(humanYears, dogYears) <==> post_gen(humanYears, dogYears)
    {
    }