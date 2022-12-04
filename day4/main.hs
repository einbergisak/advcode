import Data.List ( intersect )
import Data.Char ( isNumber )
import Data.Array ( Ix(range) )

main :: IO ()
main = do
    readFile "input.txt" >>= print . solve1 . lines
    readFile "input.txt" >>= print . solve2 . lines

solve1 :: [String] -> Int
solve1 x = length $ filter rangeFullyContained $ map (withIntersection . intoTwoSets) x
    where
        firstRange y = head y
        secondRange y = y !! 1
        withIntersection (a, b) = ((a, b), a `intersect` b)
        rangeFullyContained ((r1, r2), isec) = r1 == isec || r2 == isec

solve2 :: [String] -> Int
solve2 x = length . filter intersects $ map intoTwoSets x
    where intersects (a, b) = not (null (a `intersect` b))


intoTwoSets :: String -> ([Int], [Int])
intoTwoSets x = (fromRange (head ranges), fromRange (ranges !! 1))
    where
        len = length $ takeWhile (/= ',') x :: Int
        ranges = [take len x, drop (len+1) x]
        fromRange y = let firstPart = takeWhile isNumber y in range (read firstPart :: Int,
            read (drop (length firstPart + 1) y) :: Int)
