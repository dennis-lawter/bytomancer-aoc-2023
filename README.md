# Bytomancer's Advent Of Code 2023 Solutions

## SPOILERS WITHIN

By opening the `src/solutions` folder,
you will face implementation spoilers for the **Advent of Code 2023**.
If you wish to try the event for yourself,
please visit https://www.adventofcode.com/

## About

I've decided to tackle the Advent of Code for 2023 again this year using Rust.

## AOC Framework

This project began in November 2022,
as I worked on solving the AOC 2021 problems.
From my time tinkering with these problems,
I decided to add on a few features to ease development
(and for the simple fun of it).

### Features

1. I've implemented an input downloader which retrieves input files via the web.
   - A `.env` file is required with `SESSION=<Session ID from your cookie>`.
   - Files are downloaded to a `_cache/` folder created in the project root.
   - If an input file is already found locally, that file is loaded instead.
2. Final submissions are sent automatically to the form.
   - Using the same `.env` as above,
     executing the program with the `-s` or `--submit`
     option will send the result to the website's submission URL.
   - The resulting page is scanned and outputs a result to the command line.
3. Arguments dictate the solution to be run.
   - After discovering significant re-use between the days,
     I decided to package my code together in a single project.
   - Execution is performed with `cargo run -- dXsY`,
     representing day X solution Y.
4. Colorization is used heavily.
   - Tracking outputs and debugging is much simpler,
     thanks to the `colored` crate.

## Personal Leaderboard

```
      --------Part 1--------   --------Part 2--------
Day       Time   Rank  Score       Time   Rank  Score
 20   01:52:14   3338      0          -      -      -
 19   00:48:15   3285      0          -      -      -
 18   00:52:30   2982      0   23:00:17  15543      0
 17   12:40:10  10748      0   20:05:04  13459      0
 16   00:47:57   2949      0   00:57:06   2702      0
 15   00:11:25   3984      0   01:09:25   5595      0
 14   00:13:00   1799      0   00:42:13   1364      0
 13   01:28:25   6004      0   02:53:11   6518      0
 12   00:45:40   3253      0   12:51:55  10203      0
 11   00:29:57   3816      0   00:41:54   3500      0
 10   00:39:33   2253      0   01:37:57   1782      0
  9   00:15:55   2883      0   00:20:40   2723      0
  8   00:24:12   6801      0   00:47:38   3565      0
  7   01:08:29   7694      0   01:26:38   6044      0
  6   00:31:40   9395      0   00:38:07   8734      0
  5   00:55:42   7196      0   01:12:33   2046      0
  4   00:20:12   7543      0   00:30:52   4546      0
  3   00:53:42   6161      0   02:07:17   9302      0
  2   00:17:09   3551      0   00:24:00   3821      0
  1   00:16:39   7347      0   00:35:25   3991      0
```
