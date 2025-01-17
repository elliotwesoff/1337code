#include <stdlib.h>
#include <stdio.h>
#include "semaphore.h"
#include "stdbool.h"
#include "pthread.h"

typedef struct {
    int n;
    int i;
    bool finished;
    sem_t fizzSem, buzzSem, fizzBuzzSem, numberSem;
} FizzBuzz;

FizzBuzz* fizzBuzzCreate(int n) {
    FizzBuzz* obj = (FizzBuzz*) malloc(sizeof(FizzBuzz));
    obj->n = n;
    obj->i = 1;
    obj->finished = false;
    sem_init(&obj->fizzSem, 0, 0);
    sem_init(&obj->buzzSem, 0, 0);
    sem_init(&obj->fizzBuzzSem, 0, 0);
    sem_init(&obj->numberSem, 0, 1);
    return obj;
}

void printFizz() { printf("fizz\n"); };
void printBuzz() { printf("buzz\n"); };
void printFizzBuzz() { printf("fizzbuzz\n"); };
void printNumber(int i) { printf("%i\n", i); };

bool fizzPred(int i) {
  return i % 3 == 0 && i % 5 != 0;
}

bool buzzPred(int i) {
  return i % 5 == 0 && i % 3 != 0;
}

bool fizzbuzzPred(int i) {
  return i % 3 == 0 && i % 5 == 0;
}

bool numberPred(int i) {
  return i % 3 != 0 && i % 5 != 0;
}

void releaseAll(FizzBuzz* fbObj) {
  fbObj->finished = true;
  sem_post(&fbObj->fizzSem);
  sem_post(&fbObj->buzzSem);
  sem_post(&fbObj->fizzBuzzSem);
  sem_post(&fbObj->numberSem);
}

sem_t* getNextSem(FizzBuzz* fbObj) {
  if (fizzPred(fbObj->i)) {
    return &fbObj->fizzSem;
  } else if (buzzPred(fbObj->i)) {
    return &fbObj->buzzSem;
  } else if (fizzbuzzPred(fbObj->i)) {
    return &fbObj->fizzBuzzSem;
  } else if (numberPred(fbObj->i)) {
    return &fbObj->numberSem;
  } else {
    exit(1); // should not be reached
  }
}

void acquireAndAction(FizzBuzz* fbObj, bool (*predFn)(int), void (*outputFn)(void), sem_t* sem) {
  while (fbObj->i <= fbObj->n) {
    sem_wait(sem);

    if (fbObj->finished)
      return;

    while (predFn(fbObj->i)) {
      if (fbObj->i > fbObj->n) {
        releaseAll(fbObj);
        return;
      }

      if (numberPred(fbObj->i)) {
        printNumber(fbObj->i);
      } else {
        outputFn();
      }

      fbObj->i++;
    }

    sem_t* next = getNextSem(fbObj);
    sem_post(next);
  }
}

// printFizz() outputs "fizz".
void fizz(FizzBuzz* obj) {
  acquireAndAction(obj, fizzPred, printFizz, &obj->fizzSem);
}

// printBuzz() outputs "buzz".
void buzz(FizzBuzz* obj) {
  acquireAndAction(obj, buzzPred, printBuzz, &obj->buzzSem);
}

// printFizzBuzz() outputs "fizzbuzz".
void fizzbuzz(FizzBuzz* obj) {
  acquireAndAction(obj, fizzbuzzPred, printFizzBuzz, &obj->fizzBuzzSem);
}

// You may call global function `void printNumber(int x)`
// to output "x", where x is an integer.
void number(FizzBuzz* obj) {
  acquireAndAction(obj, numberPred, printFizz, &obj->numberSem);
}

void fizzBuzzFree(FizzBuzz* obj) {
  free(obj);
}

int main(int argc, char *argv[])
{
  pthread_t t1, t2, t3, t4;

  FizzBuzz* fb = fizzBuzzCreate(15);

  pthread_create(&t1, NULL, (void *)fizz, fb);
  pthread_create(&t2, NULL, (void *)buzz, fb);
  pthread_create(&t3, NULL, (void *)fizzbuzz, fb);
  pthread_create(&t4, NULL, (void *)number, fb);

  pthread_join(t1, NULL);
  pthread_join(t2, NULL);
  pthread_join(t3, NULL);
  pthread_join(t4, NULL);

  return EXIT_SUCCESS;
}
