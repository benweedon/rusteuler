initSidebarItems({"fn":[["is_palindromic","We use log base 10 to determine the number of digits in the number. We then iterate over the first half of the digits. We get the current digit as well as the corresponding mirrored, digit, and check if they're equal."],["is_prime","If `n` is less than or equal to 1, it isn't prime. If it's in the range `(1, 3]`, it is prime. If it's divisible by 2 or 3, it's not prime."]],"struct":[["PrimeIter","An iterator which iterates over all primes by simply incrementing an internal count and calling [`is_prime`] to check if it should be returned."]]});