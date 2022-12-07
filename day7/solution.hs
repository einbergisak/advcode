import Data.List
import Data.IntSet (isSubsetOf)
import Data.Char
main :: IO ()
main = do
    file <- readFile "input.txt" 
    print $ sum . filter (<= 100000) . solve [0] [] $ lines file
    print $ minimum . filter (>= spaceToFree file) . solution $ file
        where 
            solution = solve [0] [] . lines
            spaceToFree x = 30000000 - (70000000 - last (solution x))
        

solve :: [Int] -> [Int] -> [String] -> [Int]
solve summ parents [] = summ ++ parents
solve summ parents (line:xs)
    | isSubsequenceOf "cd .." line = solve (head parents:summ) (tail parents) xs
    | isSubsequenceOf "cd" line = solve summ updatedParents (drop (length linesToSkip + 1) xs)
    | otherwise = error "should not happen"
        where
            linesToSkip = takeWhile (notElem '$') (tail xs)
            size = sum . map (\x -> read (takeWhile isNumber x) :: Int) $ filter (not . isSubsequenceOf "dir ") linesToSkip
            updatedParents = map (+ size) (0:parents)
