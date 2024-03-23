"""
Advent of Code 2023
--- Day 17: Clumsy Crucible ---
https://adventofcode.com/2023/day/17

Vizualisation of the solution of day 17.
"""
from enum import Enum
import os
import sys
import time
from typing import Any, Callable, List, Dict, NamedTuple, Optional, Set, Tuple
import heapq



Direction = Tuple[int, int]
Coord = Tuple[int, int]
Element = Tuple[Coord, Direction, int]
UP = (0, -1)
RIGHT = (1, 0)
DOWN = (0, 1)
LEFT = (-1, 0)




def read_input(filename: str) -> List[List[int]]:
    with open(filename, "r") as f:
        return [list(map(int, list(line.strip()))) for line in f.readlines()]



def find_shortest_path(
    matrix: List[List[int]],
    max_direction_count: int = 3,
    start: Coord = (0, 0),
    end: Optional[Coord] = None,
):
    if end is None:
        end = (len(matrix[0]) - 1, len(matrix) - 1)
    candidates: List[Tuple[int, Element, List[Coord]]] = []
    for dir in [UP, RIGHT, DOWN, LEFT]:
        heapq.heappush(candidates, (0, (start, dir, 1), [start]))
    certains: Set[Element] = set()
    iteration = 0
    while candidates:
        iteration += 1
        if iteration % 100_000 == 0:
            print(f"Iteration {iteration}")

        cost, current, path = heapq.heappop(candidates)

        if current in certains:
            continue
        certains.add(current)

        new_coord = (current[0][0] + current[1][0], current[0][1] + current[1][1])
        if new_coord[0] < 0 or new_coord[0] >= len(matrix[0]) or new_coord[1] < 0 or new_coord[1] >= len(matrix):
            continue
        new_cost = cost + matrix[new_coord[1]][new_coord[0]]
        
        
        if current[2] <= max_direction_count and new_coord == end:
            print("Total iterations ", iteration)
            return new_cost
        for new_direction in [UP, RIGHT, DOWN, LEFT]:
            if new_direction[0] == 0 - current[1][0] and new_direction[1] == 0 - current[1][1]:
                continue
            new_direction_count = current[2] + 1 if new_direction == current[1] else 1
            if new_direction_count > max_direction_count :
                continue
            if (new_coord, new_direction, new_direction_count) not in certains:
                heapq.heappush(
                    candidates, (new_cost, (new_coord, new_direction, new_direction_count), path + [new_coord])
                )
                

    return -1





def main():
    input_filename = sys.argv[1]
    # input_filename = f"day_{DAY}_input_sample.txt"
    matrix = read_input(input_filename)
    res = find_shortest_path(matrix)
    print(res)



if __name__ == "__main__":
    main()