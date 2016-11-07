#!/usr/bin/env python3

import sys
import math

CELL_SIZE = 64

def debug(*args):
    print(file = sys.stderr, *args)

class StructureType:
    def __init__(self, name, size, max_health):
        self.name = name
        self.size = size
        self.max_health = max_health

class UnitType:
    def __init__(self, name, radius, max_health, speed):
        self.name = name
        self.radius = radius
        self.max_health = max_health
        self.speed = speed

STRUCTURE_TYPE = [
        StructureType("HQ", (3, 2), 350),
        StructureType("Outpost", (2, 2), 150),
        StructureType("Metal", (1, 1), 0)
]

UNIT_TYPE = [
        UnitType("Worker", 15., 15, 10.)
]

class Structure:
    def __init__(self, sid, kind, owner, x, y, health, resources):
        self.sid = sid
        self.kind = kind
        self.owner = owner
        self.loc = (x, y)
        self.health = health
        self.resources = resources

    @classmethod
    def parse(cls, line):
        return cls(*[int(i) for i in line.split()])

    def middle_point(self):
        tl = loc_to_pos(self.loc)
        size = STRUCTURE_TYPE[self.kind].size
        s = (size[0] * CELL_SIZE, size[1] * CELL_SIZE)
        return (tl[0] + s[0] / 2, tl[1] + s[1] / 2)
    
    def __repr__(self):
        return "<Structure id={} kind={} owner={} loc={} health={}/{} resources={}>".format(self.sid, STRUCTURE_TYPE[self.kind].name, self.owner, self.loc, self.health, STRUCTURE_TYPE[self.kind].max_health, self.resources)

class Unit:
    def __init__(self, uid, kind, owner, x, y, health, resources):
        self.uid = uid
        self.kind = kind
        self.owner = owner
        self.pos = (x, y)
        self.health = health
        self.resources = resources
    
    @classmethod
    def parse(cls, line):
        return cls(*[int(i) for i in line.split()])

    def __repr__(self):
        return "<Unit id={} kind={} owner={} pos={} health={}/{} resources={}>".format(self.uid, UNIT_TYPE[self.kind].name, self.owner, self.pos, self.health, UNIT_TYPE[self.kind].max_health, self.resources)

def loc_to_pos(loc):
    return (loc[0] * CELL_SIZE, loc[1] * CELL_SIZE)

def distance(a, b):
    dx, dy = b[0] - a[0], b[1] - a[1]
    return math.sqrt(dx * dx + dy * dy)

my_id = int(input())

[w, h] = [int(i) for i in input().split()]
world = []
for y in range(h):
    world.append([int(c) for c in input()])

debug(world)

while True:
    metal = int(input())
    structure_count = int(input())
    structures = [Structure.parse(input()) for i in range(structure_count)]
    unit_count = int(input())
    units = [Unit.parse(input()) for i in range(unit_count)]
    cmds = []
    for unit in units:
        if unit.owner != my_id:
            continue
        if unit.resources == 0:
            closest = min((s for s in structures if STRUCTURE_TYPE[s.kind].name == "Metal"), key = lambda s: distance(s.middle_point(), unit.pos))
            (x, y) = closest.middle_point()
            cmds.append(("MOVETO", unit.uid, math.floor(x), math.floor(y)))
        elif unit.resources == 100:
            closest = min((s for s in structures if STRUCTURE_TYPE[s.kind].name == "HQ"), key = lambda s: distance(s.middle_point(), unit.pos))
            (x, y) = closest.middle_point()
            cmds.append(("MOVETO", unit.uid, math.floor(x), math.floor(y)))
    print(len(cmds))
    for cmd in cmds:
        print(*cmd)
    debug(structures, units)
