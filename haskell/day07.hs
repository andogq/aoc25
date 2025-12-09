{- HLINT ignore "Move filter" -}
import Data.List (sort)
import Debug.Trace (trace)
import Distribution.Utils.Generic (fstOf3)

main :: IO ()
main = interact $ \input -> show (part1 input, part2 input)

padTil :: Int -> [Bool] -> [Bool]
padTil i list
  | length list >= i = list
  | otherwise = padTil i (list ++ [False])

dbgButHaskellIsShit :: (Show a) => a -> a
dbgButHaskellIsShit v = trace (show v) v

part1 :: String -> String
part1 =
  show
    . snd
    . ( \(x : xs) ->
          foldl
            ( \(beams, count) is_splitters ->
                ( padTil (length beams)
                    $ foldl
                      ( \next_beams i ->
                          let shit = padTil i next_beams
                           in if length shit > i then shit else shit ++ [True]
                      )
                      []
                    $ sort
                    $ filter (\i -> i >= 0 && i < length beams)
                    $ concatMap
                      ( \(_, is_splitter, i) ->
                          if is_splitter
                            then
                              [i - 1, i + 1]
                            else
                              [i]
                      )
                    $ filter fstOf3
                    $ zip3 beams is_splitters [0 ..],
                  count + length (filter (uncurry (&&)) $ zip beams is_splitters)
                )
            )
            (map (== 'S') x, 0)
            (map (map (== '^')) xs)
      )
    . lines

part2 :: String -> String
part2 =
  show
    . sum
    . ( \(x : xs) ->
          foldl
            ( \beam_counts is_splitters ->
                foldl
                  ( \beam_counts (i, d_beam_count) ->
                      take i beam_counts ++ [(beam_counts !! i) + d_beam_count] ++ drop (i + 1) beam_counts
                  )
                  (map (const 0) [0 .. length beam_counts])
                  $ sort
                  $ filter (\(i, _) -> i >= 0 && i < length beam_counts)
                  $ concatMap
                    ( \(beam_count, is_splitter, i) ->
                        if is_splitter
                          then
                            [(i - 1, beam_count), (i + 1, beam_count)]
                          else
                            [(i, beam_count)]
                    )
                  $ filter (\(beam_count, _, _) -> beam_count > 0)
                  $ zip3 beam_counts is_splitters [0 ..]
            )
            (map (\x -> if x == 'S' then 1 else 0) x)
            (map (map (== '^')) xs)
      )
    . lines
