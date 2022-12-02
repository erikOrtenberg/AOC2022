module Main where

import Data.List
import Data.List.Split (splitOn)

main :: IO()
main = do
    input <- readFile "day1.txt"
    let elves = map lines $ init $ splitOn "\n\n" input  
    let readElves = map (\list -> map read list) elves :: [[Integer]]
    let elvesTotalValue = map (\list -> foldl1 (+) list) readElves
    part1 elvesTotalValue
    part2 elvesTotalValue

part1 :: [Integer] -> IO()
part1 input = putStrLn $ show (maximum input)

part2 :: [Integer] -> IO()
part2 input = putStrLn $ show (foldl1 (+) (take 3 sorted)) 
    where
        sorted = reverse $ sort input
