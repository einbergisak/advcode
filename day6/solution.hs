import Data.List
main :: IO ()
main = do
    line <- readFile "input.txt"
    print $ map (\n -> solve n (reverse $ take (n-1) line) (drop (n-1) line)) sizes
        where sizes = [4, 14]

solve :: Int -> String -> String -> Int
solve _ acc [] = 0
solve n acc (x:xs) = let newAcc = x:acc in if length (nub $ take n newAcc) == n then length acc + 1 else solve n newAcc xs
