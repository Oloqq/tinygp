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
TSIZE = 2
PMUT_PER_NODE  = 0.05
# CROSSOVER_PROB = 0.9
CROSSOVER_PROB = 0.5

import random
from datetime import datetime
from copy import deepcopy
from dataclasses import dataclass

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
                "\nTSIZE="+str(TSIZE)+
                "\n----------------------------------\n")

class TinyGP:
    def __init__(self, filename: str, set_seed: int|None):
        self.fitness: list[float] = [0.0 for _ in range(POPSIZE)]
        self.x: list[float] = [0.0] * FSET_START
        self.cursor = 0
        self.targets: list[list[float]] = []
        self.varnumber: int
        self.fitnesscases: int
        self.randomnumber: int
        self.fbestpop = 0.0
        self.program: str
        self.buffer: list[chr] = ['\0'] * MAX_LEN

        set_seed = set_seed or datetime.now().timestamp()
        random.seed(set_seed)

        self.params = self.read_problem(filename)
        self.params.seed = set_seed

        for i in range(FSET_START):
            self.x[i] = random.random() * (self.params.maxrandom-self.params.minrandom) + self.params.minrandom

        self.population: list[str] = self.random_population(POPSIZE, DEPTH, self.fitness)
        self.stats(0)

    def run(self) -> float:
        primitive: str = ord(self.program[self.cursor])
        self.cursor += 1
        if ( primitive < FSET_START ):
            return(self.x[primitive])
        if primitive == ADD:
            return( self.run() + self.run() )
        elif primitive == SUB:
            return( self.run() - self.run() )
        elif primitive == MUL:
            return( self.run() * self.run() )
        elif primitive == DIV:
            num = self.run()
            den = self.run()
            if ( abs( den ) <= 0.001 ):
                return( num )
            else:
                return( num / den );
        raise Exception("run should never get here")

    def traverse(self, buffer: str, buffercount: int ) -> int:
        if ord(buffer[buffercount]) < FSET_START:
            buffercount += 1
            return( buffercount )

        if ord(buffer[buffercount]) in [ADD, SUB, MUL, DIV]:
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

    def fitness_function(self, Prog: str) -> float:
        fit = 0.0

        for i in range(self.fitnesscases):
            for j in range(self.varnumber):
                self.x[j] = self.targets[i][j]
            self.program = Prog
            self.cursor = 0
            result = self.run()
            fit += abs( result - self.targets[i][self.varnumber])
        return(-fit)

    def grow(self, buffer: str, pos: int, max: int, depth: int) -> int:
        prim = random.randint(0, 1)
        one_child: int

        if pos >= max:
            return -1

        if pos == 0:
           prim = 1

        if ( prim == 0 or depth == 0 ):
            prim = chr(random.randint(0, self.varnumber + self.randomnumber - 1))
            buffer[pos] = prim
            return pos+1
        else:
            prim = chr((random.randint(0, FSET_END - FSET_START) + FSET_START))
            if ord(prim) in [ADD, SUB, MUL, DIV]:
                buffer[pos] = prim
                one_child = self.grow(buffer, pos+1, max,depth-1)
                if ( one_child < 0 ):
                    return( -1 )
                return( self.grow( buffer, one_child, max,depth-1 ) )
        raise Exception("grow should never get here")

    def print_indiv(self, buffer: str, buffercounter: int ) -> int:
        a1=0
        a2: int
        if ord(buffer[buffercounter]) < FSET_START:
            if ( ord(buffer[buffercounter]) < self.varnumber ):
                print( "X" + str(ord(buffer[buffercounter]) + 1) + " ", end="")
            else:
                print( self.x[ord(buffer[buffercounter])], end="")
            buffercounter += 1
            return( buffercounter )
        comp = ord(buffer[buffercounter])
        if comp == ADD:
            print( "(", end="")
            buffercounter += 1
            a1=self.print_indiv( buffer, buffercounter )
            print( " + ", end="");
        if comp == SUB:
            print( "(", end="");
            buffercounter += 1
            a1=self.print_indiv( buffer, buffercounter );
            print( " - ", end="");
        if comp == MUL:
            print( "(", end="");
            buffercounter += 1
            a1=self.print_indiv( buffer, buffercounter );
            print( " * ", end="");
        if comp == DIV:
            print( "(", end="");
            buffercounter += 1
            a1=self.print_indiv( buffer, buffercounter );
            print( " / ", end="");

        a2=self.print_indiv( buffer, a1 );
        print( ")", end="");
        return( a2);

    def create_random_indiv(self, depth: int) -> list[chr]:
        length: int = self.grow(self.buffer, 0, MAX_LEN, depth)
        while length < 0:
            length = self.grow(self.buffer, 0, MAX_LEN, depth)
        return deepcopy(self.buffer)


    def random_population(self, n: int, depth: int, fitness: list[float] ) -> list[str]:
        population: list[str] = [""] * n
        print("creating population")
        for i in range(n):
            population[i] = self.create_random_indiv(depth)
            fitness[i] = self.fitness_function(population[i])
        print("population created")
        return population


    def stats(self, gen: int):
        best = random.randint(0, POPSIZE - 1)
        node_count = 0
        self.fbestpop = self.fitness[best]
        favgpop = 0.0

        for i in range(POPSIZE):
            node_count += self.traverse( self.population[i], 0 )
            favgpop += self.fitness[i]
            if ( self.fitness[i] > self.fbestpop ):
                best = i
                self.fbestpop = self.fitness[i]
        avg_len = float(node_count) / POPSIZE
        favgpop /= POPSIZE
        print("Generation="+str(gen)+" Avg Fitness="+str(-favgpop)+
                " Best Fitness="+str(-self.fbestpop)+" Avg Size="+str(avg_len)+
                "\nBest Individual: ")

        self.print_indiv( self.population[best], 0 )
        print( "\n");

    def tournament(self, fitness: list[float], tsize: int ) -> int:
        best: int = random.randint(0, POPSIZE - 1)
        competitor: int
        fbest: float = -1.0e34

        for _ in range(tsize):
            competitor = random.randint(0, POPSIZE - 1)
            if ( fitness[competitor] > fbest ):
                fbest = fitness[competitor]
                best = competitor
        return( best )

    def negative_tournament(self,  fitness: list[float], tsize: int ) -> int:
        worst = random.randint(0, POPSIZE - 1)
        competitor: int
        fworst = 1e34

        for _ in range(tsize):
            competitor = random.randint(0, POPSIZE - 1)
            if ( fitness[competitor] < fworst ):
                fworst = fitness[competitor];
                worst = competitor;
        return( worst )

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

    def mutation(self, parent: str, pmut: float ) -> str:
        len = self.traverse( parent, 0 )
        mutsite: int
        parentcopy = list(parent)

        for i in range(len):
            if ( random.random() < pmut ):
                mutsite =  i;
                if ( ord(parentcopy[mutsite]) < FSET_START ):
                    parentcopy[mutsite] = chr(random.randint(0, self.varnumber+self.randomnumber-1))
                else:
                    if ord(parentcopy[mutsite]) in [ADD, SUB, MUL, DIV]:
                        parentcopy[mutsite] = chr(random.randint(0, FSET_END - FSET_START) + FSET_START)
        return( "".join(parentcopy) )

    def evolve(self):
        print("-- TINY GP (Python version) --\n")
        print(self.params)
        gen: int = 0
        offspring: int
        parent1: int
        parent2: int
        parent: int
        newfit: float
        newind: str
        self.stats(0 )
        for gen in range(1, GENERATIONS):
            if (  self.fbestpop > -1e-5 ):
                print("PROBLEM SOLVED\n");
                exit( 0 );
            for _ in range(POPSIZE):
                if ( random.random() < CROSSOVER_PROB  ):
                    parent1 = self.tournament( self.fitness, TSIZE );
                    parent2 = self.tournament( self.fitness, TSIZE );
                    newind = self.crossover( self.population[parent1], self.population[parent2] );
                else:
                    parent = self.tournament( self.fitness, TSIZE );
                    newind = self.mutation( self.population[parent], PMUT_PER_NODE );
                newfit = self.fitness_function( newind );
                offspring = self.negative_tournament( self.fitness, TSIZE );
                self.population[offspring] = newind
                self.fitness[offspring] = newfit;
            self.stats(gen );
        print("PROBLEM *NOT* SOLVED\n");
        exit( 1 );

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