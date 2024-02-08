predicate pre_original(nums: array<int>, target: int, i:int, j:int)
reads nums
{
(nums.Length > 1) && (exists i,j::0 <= i < j < nums.Length &&  nums[i] + nums[j] == target)
}
predicate pre_gen(nums: array<int>, target: int, i:int, j:int)
reads nums
{
(nums != null && nums.Length > 1) && (exists a: int, b: int :: a < b < nums.Length && nums[a] + nums[b] == target)
}
lemma pre_eq(nums: array<int>, target: int, i:int, j:int)
ensures pre_original(nums,target,i,j) <==> pre_gen(nums,target,i ,j)
{
}
ghost predicate post_original(nums: array<int>, target: int, i:int, j:int)
requires pre_original(nums,target,i, j)
reads nums
{
(0 <= i < j < nums.Length && nums[i] + nums[j] == target) && (forall ii,jj:: (0 <= ii < i && ii < jj < nums.Length)  ==> nums[ii] + nums[jj] != target) && (forall jj:: i < jj < j ==> nums[i] + nums[jj] != target)
}
ghost predicate post_gen(nums: array<int>, target: int, i:int, j:int)
requires pre_original(nums,target,i, j)
reads nums
{
(0 <= i < j < nums.Length) && (nums[i] + nums[j] == target) && (forall k: int :: 0 <= k < i ==> !(exists l: int :: k < l < nums.Length && nums[k] + nums[l] == target)) && (forall l: int :: i < l < j ==> !(nums[i] + nums[l] == target))
}
lemma post_eq(nums: array<int>, target: int, i:int, j:int)
requires pre_original(nums,target,i,j)
requires pre_gen(nums,target,i,j)
ensures post_original(nums,target,i,j) <==> post_gen(nums,target,i,j)
{
}
