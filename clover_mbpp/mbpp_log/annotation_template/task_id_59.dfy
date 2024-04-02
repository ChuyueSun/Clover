twostate predicate pre_original(n: int, octagonalNumber: int)
    {
    true
    }

    twostate predicate pre_gen(n: int, octagonalNumber: int)
    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(n: int, octagonalNumber: int)
    ensures pre_original(n, octagonalNumber) <==> pre_gen(n, octagonalNumber)
    {
    }

    twostate predicate post_original(n: int, octagonalNumber: int)
    requires pre_original(n, octagonalNumber)
    {
    true
    }

    twostate predicate post_gen(n: int, octagonalNumber: int)
    requires pre_original(n, octagonalNumber)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(n: int, octagonalNumber: int)
    requires pre_original(n, octagonalNumber)
    requires pre_gen(n, octagonalNumber)
    ensures post_original(n, octagonalNumber) <==> post_gen(n, octagonalNumber)
    {
    }