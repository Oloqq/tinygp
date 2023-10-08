ADD = 110
SUB = 111
MUL = 112
DIV = 113
FSET_START = ADD
FSET_END = DIV
MAX_LEN = 10000
# POPSIZE = 100000
POPSIZE = 1000
DEPTH   = 5
GENERATIONS = 100
TOURNAMENT_SIZE = 2
PMUT_PER_NODE  = 0.05
# CROSSOVER_PROB = 0.9
CROSSOVER_PROB = 0.5

import random
from datetime import datetime
from copy import deepcopy
from dataclasses import dataclass

Opcode = int
Program = list[Opcode]

@dataclass
class Params:
    seed: int
    minrandom: float
    maxrandom: float

    def __repr__(self) -> str:
        return ("SEED="+str(self.seed)+"\nMAX_LEN="+str(MAX_LEN)+
                "\nPOPSIZE="+str(POPSIZE)+"\nDEPTH="+str(DEPTH)+
                "\nCROSSOVER_PROB="+str(CROSSOVER_PROB)+
                "\nPMUT_PER_NODE="+str(PMUT_PER_NODE)+
                "\nMIN_RANDOM="+str(self.minrandom)+
                "\nMAX_RANDOM="+str(self.maxrandom)+
                "\nGENERATIONS="+str(GENERATIONS)+
                "\nTSIZE="+str(TOURNAMENT_SIZE)+
                "\n----------------------------------\n")

def random_operation() -> Opcode:
    return random.randint(0, FSET_END - FSET_START) + FSET_START

def tournament(fitness: list[float], tournament_size: int) -> int:
    best: int = random.randint(0, len(fitness) - 1)
    best_fitness: float = -1.0e34

    for _ in range(tournament_size):
        competitor = random.randint(0, len(fitness) - 1)
        if fitness[competitor] > best_fitness:
            best_fitness = fitness[competitor]
            best = competitor
    return best

def negative_tournament( fitness: list[float], tsize: int ) -> int:
    worst = random.randint(0, len(fitness) - 1)
    worst_fitness = 1e34

    for _ in range(tsize):
        competitor = random.randint(0, len(fitness) - 1)
        if fitness[competitor] < worst_fitness:
            worst_fitness = fitness[competitor]
            worst = competitor
    return worst

class Executor():
    def __init__(self, program: Program, variables):
        self.program = program
        self.cursor = 0
        self.variables = variables

    def advance(self) -> float:
        primitive: Opcode = self.program[self.cursor]
        self.cursor += 1
        if ( primitive < FSET_START ):
            return(self.variables[primitive])

        if primitive == ADD:
            return( self.advance() + self.advance() )
        elif primitive == SUB:
            return( self.advance() - self.advance() )
        elif primitive == MUL:
            return( self.advance() * self.advance() )
        elif primitive == DIV:
            num = self.advance()
            den = self.advance()
            if ( abs( den ) <= 0.001 ):
                return( num )
            else:
                return( num / den )
        raise Exception("run should never get here")

def execute(program: Program, x) -> float:
    return Executor(program, x).advance()

class TinyGP:
    def __init__(self, filename: str, set_seed: int|None):
        self.fitness: list[float] = [0.0 for _ in range(POPSIZE)]
        self.targets: list[list[float]] = []
        self.varnumber: int
        self.fitnesscases: int
        self.randomnumber: int
        self.buffer: Program = [0] * MAX_LEN

        self.generation = 0

        set_seed = set_seed or datetime.now().timestamp()
        random.seed(set_seed)

        self.params = self.read_problem(filename)
        self.params.seed = set_seed

        self.variables: list[float] = [0.0] * FSET_START
        for i in range(FSET_START):
            self.variables[i] = random.random() * (self.params.maxrandom-self.params.minrandom) + self.params.minrandom

        self.population: list[Program] = self.random_population(POPSIZE, DEPTH, self.fitness)

    def traverse(self, buffer: str, buffercount: int ) -> int:
        if buffer[buffercount] < FSET_START:
            buffercount += 1
            return buffercount

        if buffer[buffercount] in [ADD, SUB, MUL, DIV]:
            buffercount += 1
            return self.traverse(buffer, self.traverse(buffer, buffercount))

        raise Exception("run should never get here")


    def read_problem(self, fname: str) -> Params:
        try:
            line: str
            file = open(fname)
            line = file.readline()
            tokens = line.split()
            self.varnumber = int(tokens[0])
            self.randomnumber = int(tokens[1])
            minrandom =	float(tokens[2])
            maxrandom =  float(tokens[3])
            self.fitnesscases = int(tokens[4])
            self.targets = [[0.0] * (self.varnumber+1) for _ in range(self.fitnesscases)]
            for i in range(self.fitnesscases):
                line = file.readline()
                tokens = line.split()
                for j in range(self.varnumber + 1):
                    t = tokens[j]
                    self.targets[i][j] = float(t)
            file.close()
        except FileExistsError as e:
            print("ERROR: Please provide a data file")
            exit(1)
        except Exception as e:
            print("ERROR: Incorrect data format")
            print(e)
            exit(1)
        return Params(None, minrandom, maxrandom)

    def fitness_function(self, prog: Program) -> float:
        fit = 0.0
        for i in range(self.fitnesscases):
            self.variables[:self.varnumber] = self.targets[i][:self.varnumber]
            result = execute(prog, self.variables)
            fit += abs( result - self.targets[i][self.varnumber])
        return -fit

    def grow(self, buffer: str, pos: int, max: int, depth: int) -> int:
        if pos >= max:
            return -1

        tmp = random.randint(0, 1) # TEMP for keeping seed
        initial = pos != 0 and tmp == 0

        if depth == 0 or initial:
            new_prim = random.randint(0, self.varnumber + self.randomnumber - 1)
            buffer[pos] = new_prim
            return pos+1
        else:
            new_prim = random_operation()
            assert new_prim in [ADD, SUB, MUL, DIV]
            buffer[pos] = new_prim
            one_child = self.grow(buffer, pos+1, max, depth - 1)
            if one_child < 0:
                return -1
            return self.grow(buffer, one_child, max, depth - 1)

    def print_indiv(self, buffer: str, buffercounter: int ) -> int:
        a1=0
        a2: int
        if buffer[buffercounter] < FSET_START:
            if buffer[buffercounter] < self.varnumber:
                print( "X" + str(buffer[buffercounter] + 1) + " ", end="")
            else:
                print( self.variables[buffer[buffercounter]], end="")
            buffercounter += 1
            return buffercounter
        comp = buffer[buffercounter]
        if comp == ADD:
            print( "(", end="")
            buffercounter += 1
            a1=self.print_indiv( buffer, buffercounter )
            print( " + ", end="")
        if comp == SUB:
            print( "(", end="")
            buffercounter += 1
            a1=self.print_indiv( buffer, buffercounter )
            print( " - ", end="")
        if comp == MUL:
            print( "(", end="")
            buffercounter += 1
            a1=self.print_indiv( buffer, buffercounter )
            print( " * ", end="")
        if comp == DIV:
            print( "(", end="")
            buffercounter += 1
            a1=self.print_indiv( buffer, buffercounter )
            print( " / ", end="")

        a2=self.print_indiv( buffer, a1 )
        print( ")", end="")
        return a2

    def create_random_indiv(self, depth: int) -> list[chr]:
        length: int = self.grow(self.buffer, 0, MAX_LEN, depth)
        while length < 0:
            length = self.grow(self.buffer, 0, MAX_LEN, depth)
        return deepcopy(self.buffer)


    def random_population(self, n: int, depth: int, fitness: list[float] ) -> list[str]:
        population: list[int] = [0] * n
        print("creating population")
        for i in range(n):
            population[i] = self.create_random_indiv(depth)
            fitness[i] = self.fitness_function(population[i])
        print("population created")
        return population

    def summarize_generation(self) -> float:
        best = random.randint(0, POPSIZE - 1)
        node_count = 0
        best_fitness = self.fitness[best]
        average_fitness = 0.0

        for i in range(POPSIZE):
            node_count += self.traverse( self.population[i], 0 )
            average_fitness += self.fitness[i]
            if ( self.fitness[i] > best_fitness ):
                best = i
                best_fitness = self.fitness[i]

        avg_len = float(node_count) / POPSIZE
        average_fitness /= POPSIZE

        print(f"Generation={self.generation} Avg Fitness={-average_fitness} \
                Best Fitness={-best_fitness} Avg Size={avg_len}")
        print("Best Individual: ")
        print(self.population[best])
        self.print_indiv( self.population[best], 0 )
        print()
        return best_fitness

    def crossover(self, parent1: str, parent2: str) -> str:
        xo1start: int; xo1end: int; xo2start: int; xo2end: int
        offspring: str
        len1 = self.traverse( parent1, 0 );
        len2 = self.traverse( parent2, 0 );
        lenoff: int

        xo1start = random.randint(0, len1 - 1)
        xo1end = self.traverse( parent1, xo1start );

        xo2start =  random.randint(0, len2 - 1)
        xo2end = self.traverse( parent2, xo2start );

        lenoff = xo1start + (xo2end - xo2start) + (len1-xo1end);

        offspring = [" "] * lenoff;

        offspring[0:xo1start] = parent1[0:xo1start]
        offspring[xo1start:(xo1start+(xo2end-xo2start))] = parent2[xo2start:xo2end]
        offspring[(xo1start + (xo2end - xo2start)):(xo1start + (xo2end - xo2start)) + (len1-xo1end)] = parent1[xo1end:len1]

        return( offspring )

    def mutation(self, parent: str, pmut: float ) -> Program:
        len = self.traverse( parent, 0 )
        mutsite: int
        parentcopy = list(parent)

        for i in range(len):
            if ( random.random() < pmut ):
                mutsite =  i
                if parentcopy[mutsite] < FSET_START:
                    parentcopy[mutsite] = random.randint(0, self.varnumber+self.randomnumber-1)
                else:
                    if parentcopy[mutsite] in [ADD, SUB, MUL, DIV]:
                        parentcopy[mutsite] = random.randint(0, FSET_END - FSET_START) + FSET_START
        return parentcopy

    def evolve(self):
        print("-- TINY GP (Python version) --\n")
        print(self.params)

        best_fitness = self.summarize_generation()
        for self.generation in range(1, GENERATIONS):
            if best_fitness > -1e-5:
                print("PROBLEM SOLVED\n")
                exit(0)
            for _ in range(POPSIZE):
                if random.random() < CROSSOVER_PROB:
                    parent1 = tournament(self.fitness, TOURNAMENT_SIZE)
                    parent2 = tournament(self.fitness, TOURNAMENT_SIZE)
                    child = self.crossover(self.population[parent1], self.population[parent2])
                else:
                    parent = tournament( self.fitness, TOURNAMENT_SIZE )
                    child = self.mutation( self.population[parent], PMUT_PER_NODE )
                child_index = negative_tournament( self.fitness, TOURNAMENT_SIZE )
                self.population[child_index] = child
                self.fitness[child_index] = self.fitness_function(child)
            best_fitness = self.summarize_generation()
        print("PROBLEM *NOT* SOLVED\n")
        exit(1)

def main(args):
    if len(args) == 3:
        seed = int(args[1])
        filename = args[2]
    elif len(args) == 2:
        seed = None
        filename = args[1]
    else:
        seed = None
        filename = "problem.dat"

    gp = TinyGP(filename, seed)
    gp.evolve()

import sys

if __name__ == "__main__":
    main(sys.argv)