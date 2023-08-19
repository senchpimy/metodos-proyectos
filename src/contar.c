#include "contar.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

Raices numero_de_raices(char *funcion_pr)
{ 
  int len = strlen(funcion_pr);
  char *funcion = malloc(sizeof(char)*strlen(funcion_pr)+1);
  strcpy(funcion+1, funcion_pr);

  char sig;
  if (funcion[1]=='-'){
    funcion=funcion+1;
    sig='-';
  }
  else{
    funcion[0]='+';
    sig='+';
  }

  int cambios=0;
  char* reversa=funcion;
  for (int i = 0; i < len; i++){
    if (funcion[i]=='-' || funcion[i]=='+')
      if (funcion[i]!=sig){
        cambios++;
        sig=funcion[i];
      }
  }

//  printf("Se dieron %i cambios de signo en la funcion positiva\n", cambios);

  for (int j = 0; j<len; j++){
    if (*reversa=='^'){
      int numero = *(reversa+1) - 48;
      if (numero%2!=0){ //inpar
        char *regresar=reversa;
        while (1) {
          if (*regresar=='-' || *regresar=='+'){
            if (*regresar=='-') *regresar='+';
            else *regresar='-';
            break;
          }
          regresar--;
        }
      }
    }
    reversa++;
  }
  
  //printf("string %s\n",funcion);

  int cambios_reversa=0;
  char sim_reversa=*funcion;
  for (int i = 0; i < len; i++){
    if (funcion[i]=='-' || funcion[i]=='+')
      if (funcion[i]!=sim_reversa){
        cambios_reversa++;
        sim_reversa=funcion[i];
      }
  }
  Raices res;
  res.negativas=cambios_reversa;
  res.positivas=cambios;
 // printf("Se dieron %i cambios de signo en la funcion negativa\n", cambios_reversa);

  return res;
}

int str_to_num(char* s, int index){
  char *s2 = s+index;
  int val = 0;
  while (1){
    val = val*10;
    val += *s2-48;
    if (*(s2+1) > 57)
      break;
    s2++;
  }
  return val;
} 
