twostate predicate pre_original(arr: array<int>, L: int, firstPart: seq<int>, secondPart: seq<int>)
    reads arr
    {
    true
    }

    twostate predicate pre_gen(arr: array<int>, L: int, firstPart: seq<int>, secondPart: seq<int>)
    reads arr

    {
    true // (#PRE) && ... (#PRE)
    }

    twostate lemma pre_eq(arr: array<int>, L: int, firstPart: seq<int>, secondPart: seq<int>)
    ensures pre_original(arr, L, firstPart, secondPart) <==> pre_gen(arr, L, firstPart, secondPart)
    {
    }

    twostate predicate post_original(arr: array<int>, L: int, firstPart: seq<int>, secondPart: seq<int>)
    reads arr
    requires pre_original(arr, L, firstPart, secondPart)
    {
    true
    }

    twostate predicate post_gen(arr: array<int>, L: int, firstPart: seq<int>, secondPart: seq<int>)
    reads arr
    requires pre_original(arr, L, firstPart, secondPart)
    {
    true // (#POST) && ... (#POST)
    }

    twostate lemma post_eq(arr: array<int>, L: int, firstPart: seq<int>, secondPart: seq<int>)
    requires pre_original(arr, L, firstPart, secondPart)
    requires pre_gen(arr, L, firstPart, secondPart)
    ensures post_original(arr, L, firstPart, secondPart) <==> post_gen(arr, L, firstPart, secondPart)
    {
    }